import { Storage } from "./storage";
import { Keypair, PublicKey } from "@solana/web3.js";

jest.mock("@solana/web3.js", () => {
    return {
        ...jest.requireActual("@solana/web3.js"),
        sendAndConfirmTransaction: jest.fn().mockResolvedValue("mocked_signature"),
        Connection: jest.fn().mockImplementation(() => {
            return {
                // Mock any methods you need here
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
            await expect(storage.get("invalid_entity_id")).rejects.toThrow();
        });
    });
});
