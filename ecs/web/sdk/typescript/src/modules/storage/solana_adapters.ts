import { PublicKey, Keypair, Connection, Transaction, TransactionInstruction } from '@solana/web3.js';

interface Entity {
    id: string;
    data: Record<string, unknown>; // More specific data type
}

interface Storage {
    length: number;
    migrate(): Promise<void>;
    create(entityData: Entity): Promise<void>;
    get(entityId: string): Promise<Entity>;
    set(entityId: string, data: Entity): Promise<void>;
    delete(entityId: string): Promise<void>;
}

export class SolanaAdapters implements Storage {
    private _length: number = 0;
    get length(): number {
        return this._length;
    }

    private blueprint: string = '';
    private programId: PublicKey;
    private sessionKeypair: Keypair = Keypair.generate();
    private rpcUrl: string = '';
    private connection: Connection;

    constructor(blueprintOrConnectionString: string, programId: string, rpcUrl?: string) {
        this.blueprint = blueprintOrConnectionString;
        this.programId = new PublicKey(programId);
        this.rpcUrl = rpcUrl || blueprintOrConnectionString;
        this.connection = this.initializeConnection(this.rpcUrl);
    }

    private initializeConnection(connectionString: string): Connection {
        console.log(`Connecting to Solana with connection string: ${connectionString}`);
        return new Connection(connectionString);
    }

    private getSessionKeypair(): Keypair {
        return this.sessionKeypair;
    }

    private serialize(data: any): Buffer {
        return Buffer.from(JSON.stringify(data));
    }

    public async migrate(): Promise<void> {
        console.log('Migrating Solana storage schema...');
        const sessionKeypair = this.getSessionKeypair();

        const createWorldInstruction = new TransactionInstruction({
            keys: [
                { pubkey: sessionKeypair.publicKey, isSigner: true, isWritable: true },
            ],
            programId: this.programId,
            data: this.serialize('migrate'), // Serialize the migration action
        });

        const transaction = new Transaction().add(createWorldInstruction);
        transaction.feePayer = sessionKeypair.publicKey;

        try {
            const signature = await this.connection.sendTransaction(transaction, [sessionKeypair]);
            console.log(`Migration transaction sent: ${signature}`);
            await this.connection.confirmTransaction(signature); // Confirm the transaction
        } catch (error) {
            console.error(`Migration failed: ${(error as Error).message}`);
            throw new Error(`Migration failed: ${(error as Error).message}`);
        }

        const schemaExists = await this.checkSchemaExists();
        if (!schemaExists) {
            await this.createSchema();
            console.log('Storage schema created.');
        } else {
            console.log('Storage schema already exists.');
        }
    }

    public async create(entityData: Entity): Promise<void> {
        // Check if entity already exists
        try {
            await this.get(entityData.id);
            console.log(`Entity with ID ${entityData.id} already exists. Skipping creation.`);
            return; // Skip creation if the entity already exists
        } catch (error: any) {
            if (error.message !== `No account found for entity ID: ${entityData.id}`) {
                throw error; // Rethrow if it's not a "not found" error
            }
        }
        console.log(`Creating new entity with ID: ${entityData.id} in Solana storage...`);
        const sessionKeypair = this.getSessionKeypair();

        const serializedData = this.serialize(entityData);

        const spawnEntityInstruction = new TransactionInstruction({
            keys: [
                { pubkey: sessionKeypair.publicKey, isSigner: true, isWritable: true },
            ],
            programId: this.programId,
            data: serializedData,
        });

        const transaction = new Transaction().add(spawnEntityInstruction);
        transaction.feePayer = sessionKeypair.publicKey;

        try {
            const signature = await this.connection.sendTransaction(transaction, [sessionKeypair]);
            await this.connection.confirmTransaction(signature); // Confirm the transaction
            console.log(`Entity creation transaction sent: ${signature}`);
        } catch (error) {
            console.error(`Entity creation failed for ID ${entityData.id}: ${(error as Error).message}`);
            throw new Error(`Entity creation failed for ID ${entityData.id}: ${(error as Error).message}`);
        }
    }

    public async get(entityId: string): Promise<Entity> {
        console.log(`Fetching data for entity ID: ${entityId}`);
        const accountInfo = await this.connection.getAccountInfo(new PublicKey(entityId));

        if (!accountInfo) {
            throw new Error(`No account found for entity ID: ${entityId}`);
        }

        return {
            id: entityId,
            data: JSON.parse(accountInfo.data.toString('utf-8')),
        };
    }

    public async set(entityId: string, data: Entity): Promise<void> {
        console.log(`Updating data for entity ID: ${entityId}`);
        const sessionKeypair = this.getSessionKeypair();

        const serializedData = this.serialize(data);

        const updateEntityInstruction = new TransactionInstruction({
            keys: [
                { pubkey: sessionKeypair.publicKey, isSigner: true, isWritable: true },
            ],
            programId: this.programId,
            data: serializedData,
        });

        const transaction = new Transaction().add(updateEntityInstruction);
        transaction.feePayer = sessionKeypair.publicKey;

        try {
            const signature = await this.connection.sendTransaction(transaction, [sessionKeypair]);
            await this.connection.confirmTransaction(signature); // Confirm the transaction
            console.log(`Entity update transaction sent: ${signature}`);
        } catch (error) {
            console.error(`Entity update failed for ID ${entityId}: ${(error as Error).message}`);
            throw new Error(`Entity update failed for ID ${entityId}: ${(error as Error).message}`);
        }
    }

    public async delete(entityId: string): Promise<void> {
        console.log(`Deleting entity ID: ${entityId}`);
        const sessionKeypair = this.getSessionKeypair();

        const despawnEntityInstruction = new TransactionInstruction({
            keys: [
                { pubkey: sessionKeypair.publicKey, isSigner: true, isWritable: true },
            ],
            programId: this.programId,
            data: this.serialize({ action: 'delete', id: entityId }),
        });

        const transaction = new Transaction().add(despawnEntityInstruction);
        transaction.feePayer = sessionKeypair.publicKey;

        try {
            const signature = await this.connection.sendTransaction(transaction, [sessionKeypair]);
            await this.connection.confirmTransaction(signature); // Confirm the transaction
            console.log(`Entity deletion transaction sent: ${signature}`);
        } catch (error) {
            console.error(`Entity deletion failed for ID ${entityId}: ${(error as Error).message}`);
            throw new Error(`Entity deletion failed for ID ${entityId}: ${(error as Error).message}`);
        }
    }

    private async checkSchemaExists(): Promise<boolean> {
        console.log('Checking if storage schema exists...');
        // Example logic to check if the schema exists
        const schemaAccountInfo = await this.connection.getAccountInfo(this.programId);
        return schemaAccountInfo !== null;
        return false;
    }

    private async createSchema(): Promise<void> {
        console.log('Creating storage schema...');
        // Example logic to create the storage schema
        const createSchemaInstruction = new TransactionInstruction({
            keys: [
                { pubkey: this.sessionKeypair.publicKey, isSigner: true, isWritable: true },
            ],
            programId: this.programId,
            data: this.serialize({ action: 'create_schema' }), // Serialize the schema creation action
        });

        const transaction = new Transaction().add(createSchemaInstruction);
        transaction.feePayer = this.sessionKeypair.publicKey;

        try {
            const signature = await this.connection.sendTransaction(transaction, [this.sessionKeypair]);
            await this.connection.confirmTransaction(signature); // Confirm the transaction
            console.log('Schema creation transaction sent:', signature);
        } catch (error) {
            console.error(`Schema creation failed: ${(error as Error).message}`);
            throw new Error(`Schema creation failed: ${(error as Error).message}`);
        }
    }

    public async ensureSufficientFunds(): Promise<void> {
        const sessionKeypair = this.getSessionKeypair();
        const balance = await this.connection.getBalance(sessionKeypair.publicKey);
        const requiredFeeAmount = 0.0025; // Example fee
        if (balance < requiredFeeAmount) {
            throw new Error('Insufficient balance to pay transaction fee');
        }
    }
}

// Example instantiation of the SolanaAdapter class
const adapter = new SolanaAdapters('https://api.mainnet-beta.solana.com', '5EyVYyZ1g5g5g5g5g5g5g5g5g5g5g5g5g5g5g5g5g5g');
console.log('SolanaAdapter instance created:', adapter);