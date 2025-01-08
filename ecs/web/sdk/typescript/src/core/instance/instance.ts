import { PublicKey } from '@solana/web3.js';
import { Component,ComponentValue } from '../blueprint/blueprint'; //should be the path to the component file

class Instance{
  components: Map<Component, ComponentValue>;
  nonce: number;
  instanceAuthority: PublicKey;
  bump: number;

  constructor(components: Map<Component, ComponentValue>,nonce: number,instanceAuthority: PublicKey,bump: number) {
    // Store values directly when creating an instance
    this.components = components;
    this.nonce = nonce;
    this.instanceAuthority = instanceAuthority;
    this.bump = bump;
  }
}

