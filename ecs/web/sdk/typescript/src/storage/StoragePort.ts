export interface StoragePort {
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
