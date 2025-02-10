import { SolanaAdapters } from "../storage/solanaAdapter";
import { PublicKey, RpcResponseAndContext } from "@solana/web3.js"; // Importing the necessary type
import { InstructionData } from "../storage/solanaAdapter"; // Import InstructionData for testing

describe("SolanaAdapters", () => {
    let adapter: SolanaAdapters; // Declare adapter variable

    const storeProgramAddress = "8npxEZiWoi6zcBQ4Pw2e5enC1Av4UhzA2ZtPn1fKeciU";
    const rpcUrl = "https://devnet.sonic.game";

    it("should create an instance of SolanaAdapters with correct values", () => {
        const adapter = new SolanaAdapters({
            blueprint: "testBlueprint",
            programId: new PublicKey(storeProgramAddress),
            rpcUrl: rpcUrl,
        });

        expect(adapter).toBeDefined();
        expect(adapter.getProgramId().toString()).toBe(storeProgramAddress);
        expect(adapter.getRpcUrl()).toBe(rpcUrl);
    });

    beforeEach(() => {
        adapter = new SolanaAdapters({
            blueprint: "testBlueprint",
            programId: new PublicKey(storeProgramAddress),
            rpcUrl: rpcUrl,
        });
    });

    // New tests for InstructionData serialization and deserialization
    describe("InstructionData Serialization", () => {
        it("should serialize and deserialize correctly", () => {
            const data = new InstructionData("test", 123);
            const serialized = data.serialize();
            const deserialized = InstructionData.borshDeserialize(serialized);

            expect(deserialized.property1).toBe("test");
            expect(deserialized.property2).toBe(123);
        });
    });

    describe("migrate function", () => {
        it("should successfully migrate and spawn entities", async () => {
            const sendTransactionMock = jest.spyOn(adapter['connection'], 'sendTransaction').mockResolvedValue('mockSignature');
            const confirmTransactionMock = jest.spyOn(adapter['connection'], 'confirmTransaction').mockResolvedValue({
                context: { slot: 0 },
                value: {
                    err: null,
                },
            } as RpcResponseAndContext<any>);

            await adapter.migrate();

            expect(sendTransactionMock).toHaveBeenCalled();
            expect(confirmTransactionMock).toHaveBeenCalled();
            
            // Clean up mocks
            sendTransactionMock.mockRestore();
            confirmTransactionMock.mockRestore();
        });

        it("should handle errors during migration", async () => {
            jest.spyOn(adapter['connection'], 'sendTransaction').mockRejectedValue(new Error("Transaction failed"));

            await expect(adapter.migrate()).rejects.toThrow("Transaction failed");
        });
    });

    describe("create function", () => {
        it("should successfully create a new entity", async () => {
            const sendTransactionMock = jest.spyOn(adapter['connection'], 'sendTransaction').mockResolvedValue('mockSignature');
            const confirmTransactionMock = jest.spyOn(adapter['connection'], 'confirmTransaction').mockResolvedValue({
                context: { slot: 0 },
                value: {
                    err: null,
                },
            } as RpcResponseAndContext<any>);

            await adapter.create();

            expect(sendTransactionMock).toHaveBeenCalled();
            expect(confirmTransactionMock).toHaveBeenCalled();
            
            // Clean up mocks
            sendTransactionMock.mockRestore();
            confirmTransactionMock.mockRestore();
        });

        it("should handle errors during creation", async () => {
            jest.spyOn(adapter['connection'], 'sendTransaction').mockRejectedValue(new Error("Transaction failed"));

            await expect(adapter.create()).rejects.toThrow("Transaction failed");
        });
    });

    describe("get function", () => {
        it("should successfully retrieve an entity", async () => {
            const entityId = "exampleEntityId";

            const sendTransactionMock = jest.spyOn(adapter['connection'], 'sendTransaction').mockResolvedValue('mockSignature');
            const confirmTransactionMock = jest.spyOn(adapter['connection'], 'confirmTransaction').mockResolvedValue({
                context: { slot: 0 },
                value: {
                    err: null,
                },
            } as RpcResponseAndContext<any>);

            await adapter.get(entityId);

            expect(sendTransactionMock).toHaveBeenCalled();
            expect(confirmTransactionMock).toHaveBeenCalled();
            
            // Clean up mocks
            sendTransactionMock.mockRestore();
            confirmTransactionMock.mockRestore();
        });

        it("should handle errors during retrieval", async () => {
            const entityId = "exampleEntityId";
            jest.spyOn(adapter['connection'], 'sendTransaction').mockRejectedValue(new Error("Transaction failed"));

            await expect(adapter.get(entityId)).rejects.toThrow("Transaction failed");
        });
    });

    describe("set function", () => {
        it("should successfully set/update an entity", async () => {
            const testData = { property1: "exampleProperty", property2: 123 };
            const entityId = "exampleEntityId";

            const sendTransactionMock = jest.spyOn(adapter['connection'], 'sendTransaction').mockResolvedValue('mockSignature');
            const confirmTransactionMock = jest.spyOn(adapter['connection'], 'confirmTransaction').mockResolvedValue({
                context: { slot: 0 },
                value: {
                    err: null,
                },
            } as RpcResponseAndContext<any>);

            await adapter.set(entityId, testData);

            expect(sendTransactionMock).toHaveBeenCalled();
            expect(confirmTransactionMock).toHaveBeenCalled();
            
            // Clean up mocks
            sendTransactionMock.mockRestore();
            confirmTransactionMock.mockRestore();
        });

        it("should handle errors during update", async () => {
            const entityId = "exampleEntityId";
            const testData = { property1: "exampleProperty", property2: 123 };
            jest.spyOn(adapter['connection'], 'sendTransaction').mockRejectedValue(new Error("Transaction failed"));

            await expect(adapter.set(entityId, testData)).rejects.toThrow("Transaction failed");
        });
    });

    describe("delete function", () => {
        it("should successfully delete an entity", async () => {
            const entityId = "exampleEntityId";

            const sendTransactionMock = jest.spyOn(adapter['connection'], 'sendTransaction').mockResolvedValue('mockSignature');
            const confirmTransactionMock = jest.spyOn(adapter['connection'], 'confirmTransaction').mockResolvedValue({
                context: { slot: 0 },
                value: {
                    err: null,
                },
            } as RpcResponseAndContext<any>);

            await adapter.delete(entityId);

            expect(sendTransactionMock).toHaveBeenCalled();
            expect(confirmTransactionMock).toHaveBeenCalled();
            
            // Clean up mocks
            sendTransactionMock.mockRestore();
            confirmTransactionMock.mockRestore();
        });

        it("should handle errors during deletion", async () => {
            const entityId = "exampleEntityId";
            jest.spyOn(adapter['connection'], 'sendTransaction').mockRejectedValue(new Error("Transaction failed"));

            await expect(adapter.delete(entityId)).rejects.toThrow("Transaction failed");
        });
    });

    describe("getSchema function", () => {
        it("should return the correct schema", () => {
            const schema = adapter.getSchema();
            expect(schema).toEqual({
                properties: {
                    property1: { type: "string" },
                    property2: { type: "number" },
                },
                required: ["property1", "property2"],
            });
        });
    });
});
