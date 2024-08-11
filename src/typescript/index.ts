import MerkleTree from "merkletreejs"
import keccak256 from "keccak256"

async function main() {
  const leaves = ['a', 'b'].map(x => {
    console.log('Leaf:', x)
    console.log('Hash:', keccak256(x))
    let pretty = keccak256(x).reduce((acc, byte) => {
      return acc + byte.toString() + ', '
    }, '')
    console.log('Pretty:', pretty)
    // console.log('Hash:', keccak256(x).toString('hex'))
    console.log('\n')
    return keccak256(x)
  })
  const tree = new MerkleTree(leaves, keccak256)
  const root = tree.getRoot()
  console.log('Root:', root)
  const prettyRoot = root.reduce((acc, byte) => {
    return acc + byte.toString() + ', '
  }, '')
  console.log('Pretty Root:', prettyRoot)

  const leaf = keccak256('a')
  const proof = tree.getProof(leaf)
  console.log('Proof:', proof)
  proof.forEach(proofOuter => {
    let prettyProof = proofOuter.data.reduce((acc, byte) => {
      return acc + byte.toString() + ', '
    }, '')
    console.log('Pretty Proof:', prettyProof)
  })
  const result = keccak256(Buffer.concat([leaf, proof[0].data]))
  let prettyResult = result.reduce((acc, byte) => {
    return acc + byte.toString() + ', '
  }, '')
  console.log('Pretty Result:', prettyResult)
  console.log('Result:', result.toString('hex'))
}

main().catch((error) => {
  console.error(error)
  process.exit(1)
})