// import { Storage } from '../../../types/interface_storage';

// export class MemoryAdapter implements Storage {
//     blueprint: string;
//     program_id: string;
//     rpc_url: string;
//     private instances: Map<string, any>;

//     constructor(blueprint: string, program_id: string, rpc_url: string) {
//         this.blueprint = blueprint;
//         this.program_id = program_id;
//         this.rpc_url = rpc_url;
//         this.instances = new Map();
//     }

//     async migrate(): Promise<void> {
//         // Migration logic: Check if instances need to be initialized
//         if (!this.instances.size) {
//             const initialData = { createdAt: new Date(), data: {} }; // Example structure
//             this.instances.set(this.blueprint, initialData);
//             console.log(`Migration completed. Initialized instances with blueprint: ${this.blueprint}`);
//         }
//     }

//     async create(): Promise<void> {
//         // Initialize instances based on the interface_storage requirements
//         this.instances.set(this.blueprint, { createdAt: new Date(), data: {} });
//         console.log(`Created new instance for blueprint: ${this.blueprint}`);
//     }

//     async get(entityId: string): Promise<any> {
//         const data = this.instances.get(entityId);
//         if (!data) {
//             throw new Error(`Entity with ID ${entityId} does not exist.`);
//         }
//         console.log(`Retrieved data for entity ID ${entityId}:`, data);
//         return data;
//     }

//     async set(entityId: string, data: any): Promise<void> {
//     if (!entityId) {
//         throw new Error("Entity ID cannot be empty.");
//     }

//     console.log(`Setting data for entity ID ${entityId}:`, data);

//     this.instances.set(entityId, data);
//     }

//     async delete(entityId: string): Promise<void> {
//         if (!this.instances.has(entityId)) {
//             throw new Error(`Entity with ID ${entityId} does not exist.`);
//         }
//         this.instances.delete(entityId);
//         console.log(`Deleted data for entity ID ${entityId}.`);
//     }
// }
