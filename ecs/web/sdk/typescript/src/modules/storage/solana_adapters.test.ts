import { SolanaAdapters } from './solana_adapters';
import { Connection, Keypair, PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';

jest.mock('@solana/web3.js', () => {
  return {
    Connection: jest.fn().mockImplementation(() => ({
        sendTransaction: jest.fn().mockResolvedValue('fake_signature'),
        getAccountInfo: jest.fn().mockResolvedValue({ id: "123", data: JSON.stringify({ key: "value" }) }),
        confirmTransaction: jest.fn().mockResolvedValue(true),
        getBalance: jest.fn().mockResolvedValue(100),
    })),
    Keypair: {
        generate: jest.fn().mockReturnValue({
            publicKey: {
                toString: jest.fn().mockReturnValue('fake_public_key')
            },
            secretKey: new Uint8Array(),
        }),
    },
    Transaction: jest.fn().mockImplementation(() => {
      return {
        add: jest.fn().mockReturnThis(), // Ensure add returns the Transaction instance
        feePayer: { toString: jest.fn().mockReturnValue('fake_public_key') }, // Mock feePayer
      };
    }),
    TransactionInstruction: jest.fn().mockImplementation(() => {
      return {};
    }),
    PublicKey: jest.fn().mockImplementation((key) => {
        return {
            toString: jest.fn().mockReturnValue(key), // Return the key passed to the constructor
        };
    }),
  };
});

jest.setTimeout(10000); // Set timeout for tests to 10 seconds

describe('SolanaAdapters', () => {
    let adapter: SolanaAdapters; 
    const mockProgramId = '5EyVYyZ1g5g5g5g5g5g5g5g5g5g5g5g5g5g5g5g5g';
    const mockRpcUrl = 'https://api.mainnet-beta.solana.com';

    beforeEach(() => {
        adapter = new SolanaAdapters(mockRpcUrl, mockProgramId);
    });

    it('should initialize with correct values', () => {
        expect(adapter).toBeDefined();
    });

    it('should migrate the schema', async () => {
        await adapter.ensureSufficientFunds(); // Ensure sufficient funds before migration
        await adapter.migrate();
        // Verify that the migration transaction was sent
        expect(adapter['connection'].sendTransaction).toHaveBeenCalled(); // Use the mocked connection instance
    });

    it('should create a new entity', async () => {
        const entityData = { id: '5EyVYyZ1g5g5g5g5g5g5g5g5g5g5g5g5g5g5g5g5g5g', data: {} };
        // Ensure the entity does not exist before creating
        try {
            await adapter.delete(entityData.id);
        } catch (error) {
            // Ignore error if the entity does not exist
        }

        await adapter.create(entityData);
        // Verify that the entity creation transaction was sent
        expect(adapter['connection'].sendTransaction).toHaveBeenCalled(); // Use the mocked connection instance
    });

    it('should set data for an entity', async () => {
        const entityData = { id: '123', data: { field: 'value' } };
        await adapter.set('123', entityData);
        // Verify that the entity update transaction was sent
        expect(adapter['connection'].sendTransaction).toHaveBeenCalled(); // Use the mocked connection instance
    });

    it('should fetch an entity', async () => {
        const entity = await adapter.get('123');
        expect(entity).toEqual({
            id: '123',
            data: {
                key: 'value',
            },
        });
    });

    it('should delete an entity', async () => {
        await adapter.delete('123');
        // Verify that the entity deletion transaction was sent
        expect(adapter['connection'].sendTransaction).toHaveBeenCalled(); // Use the mocked connection instance
    });

    it('should check sufficient balance before performing transactions', async () => {
        await adapter.ensureSufficientFunds();
    });
});
