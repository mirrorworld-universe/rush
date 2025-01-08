import { Storage } from "../storage/storage";
import { Keypair, PublicKey } from "@solana/web3.js";

jest.mock("@solana/web3.js", () => {
    return {
        ...jest.requireActual("@solana/web3.js"),
        sendAndConfirmTransaction: jest.fn().mockResolvedValue("mocked_signature"),
        Connection: jest.fn().mockImplementation(() => {
            return {
                getAccountInfo: jest.fn().mockResolvedValue(null),
            };
        }),
    };
});

describe("Storage", () => {
    let storage: Storage;
    let keypair: Keypair;

    beforeEach(() => {
        keypair = Keypair.generate();
        storage = new Storage({
            blueprint: "/path/to/blueprint",
            programId: keypair.publicKey.toString(),
            signer: keypair,
            rpcUrl: "http://127.0.0.1:8899",
        });
    });

    describe("get", () => {
        it("should retrieve entity data successfully", async () => {
            const entityId = keypair.publicKey.toString();
            const signature = await storage.get(entityId);
            expect(signature).toBe("mocked_signature");
        });

        it("should throw an error for invalid entityId", async () => {
            await expect(storage.get("invalid_entity_id")).rejects.toThrow(Error);
        });
    });

    describe("migrate", () => {
        it("should create an on-chain world successfully", async () => {
            const signature = await storage.migrate();
            expect(signature).toBe("mocked_signature");
        });
    });

    describe("delete", () => {
        it("should delete an entity successfully", async () => {
            const entityId = keypair.publicKey.toString();
            const signature = await storage.delete(entityId);
            expect(signature).toBe("mocked_signature");
        });
    });

    describe("set", () => {
        it("should update entity data successfully", async () => {
            const entityId = keypair.publicKey.toString();
            const data = { key: "value" };
            const signature = await storage.set(entityId, data);
            expect(signature).toBe("mocked_signature");
        });
    });
});