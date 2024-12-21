import { Storage } from '../../../types/interface_storage';



export class MemoryAdapter implements Storage {
    blueprint: string;
    program_id: string;
    rpc_url: string;
    private instances: Map<string, any>;

    constructor(blueprint: string, program_id: string, rpc_url: string) {
        this.blueprint = blueprint;
        this.program_id = program_id;
        this.rpc_url = rpc_url;
        this.instances = new Map();
    }


    async migrate(): Promise<void> {
    }

    async create(): Promise<void> {
        this.instances.clear();
    }

    async get(entityId: string): Promise<any> {
        const data = this.instances.get(entityId);
        if (!data) {
            throw new Error(`Entity with ID ${entityId} does not exist.`);
        }
        console.log(`Retrieved data for entity ID ${entityId}:`, data);
        return data;
    }


    async set(entityId: string, data: any): Promise<void> {
        this.instances.set(entityId, data);
    }


    async delete(entityId: string): Promise<void> {
        if (!this.instances.has(entityId)) {
            throw new Error(`Entity with ID ${entityId} does not exist.`);
        }
        this.instances.delete(entityId);
        console.log(`Deleted data for entity ID ${entityId}.`);
    }
}
