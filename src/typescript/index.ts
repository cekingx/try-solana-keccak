import MerkleTree from "merkletreejs"
import keccak256 from "keccak256"

const printPretty = (buffer: Buffer) => {
  let pretty = buffer.reduce((acc, byte) => {
    return acc + byte.toString() + ', '
  }, '')
  return pretty
}

async function main() {
  const bannedAddress = 'EFswvxFPzSkpPEgMuAHZrsRZsctJooUswcwYNQiMtYCH'
  const allowedAddress = [
    '5c1HBqtcU7227XAhrWtAdvVWTJvgehYo3Hp2ieNxEfM6',
    'FRn4iGBdh3r451HkdyJmWNHTa7w2HzzE2qZryEY3z9Uh'
  ]
  const leaves = allowedAddress.map(x => {
    console.log('Leaf:', x)
    console.log('pretty leaf', printPretty(keccak256(x)))
    console.log('\n')
    return keccak256(x)
  })
  const tree = new MerkleTree(leaves, keccak256)
  const root = tree.getRoot()
  console.log('pretty root', printPretty(root))
  console.log('\n')

  const leaf = keccak256(allowedAddress[0])
  const proof = tree.getProof(leaf)
  console.log('Proof:', proof)
  proof.forEach(proofInner => {
    console.log('pretty proof', printPretty(proofInner.data))
  })
  console.log('\n')

  const result = keccak256(Buffer.concat([leaf, proof[0].data]))
  console.log('pretty result', printPretty(result))
}

main().catch((error) => {
  console.error(error)
  process.exit(1)
})