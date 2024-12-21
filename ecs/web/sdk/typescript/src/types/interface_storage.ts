export interface Storage {
    /**
     * Migrates the storage schema or performs any necessary upgrades.
     */
    migrate(): Promise<void>;

    /**
     * Initializes the storage system, preparing it for use.
     */
    create(): Promise<void>;

    /**
     * Retrieves data for a given entity ID.
     * @param entityId - The unique identifier of the entity to retrieve.
     * @returns A promise that resolves with the data of the entity.
     */
    get(entityId: string): Promise<any>;

    /**
     * Stores data for a given entity ID.
     * @param entityId - The unique identifier of the entity to store.
     * @param data - The data to associate with the entity.
     * @returns A promise that resolves when the data has been stored.
     */
    set(entityId: string, data: any): Promise<void>;

    /**
     * Deletes data for a given entity ID.
     * @param entityId - The unique identifier of the entity to delete.
     * @returns A promise that resolves when the data has been deleted.
     */
    delete(entityId: string): Promise<void>;
}

export interface StoragePort {
    /**
     * Saves data for a given key.
     * @param data - The data to be saved.
     * @returns A promise that resolves when the data has been saved.
     */
    save(data: string): Promise<void>;

    /**
     * Loads data for a given key.
     * @param key - The unique identifier of the data to load.
     * @returns A promise that resolves with the loaded data.
     */
    load(key: string): Promise<string>;

    /**
     * Deletes data for a given key.
     * @param key - The unique identifier of the data to delete.
     * @returns A promise that resolves when the data has been deleted.
     */
    delete(key: string): Promise<void>;
}
