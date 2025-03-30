import { Keypair } from '@solana/web3.js';
import bs58 from 'bs58';

const PREFIX = 'anza';
let attempts = 0;

console.log(`Пошук ключа, що починається з "${PREFIX}"...`);

while (true) {
    const keypair = Keypair.generate();
    const pubkey = keypair.publicKey.toBase58();

    if (pubkey.startsWith(PREFIX)) {
        console.log(`✅ Знайдено після ${attempts} спроб:`);
        console.log('Public Key:', pubkey);
        console.log('Secret Key:', bs58.encode(keypair.secretKey));
        break;
    }

    attempts++;
}

console.log('🎉 Готово!');