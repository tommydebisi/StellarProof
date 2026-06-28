import Header from "@/components/Header";
import Link from "next/link";

export default function DocsPage() {
  return (
    <div className="min-h-screen bg-gray-50 dark:bg-[#020617]">
      <Header />
      <main className="mx-auto max-w-3xl px-4 py-10 sm:px-6 lg:px-8 prose prose-invert">
        <h1>StellarProof Documentation</h1>
        <p>
          StellarProof mints on-chain provenance certificates for digital media on
          Stellar Soroban. Phase 1 focuses on content hashing, manifest metadata,
          and certificate minting.
        </p>

        <h2>Phase 1 (current)</h2>
        <ul>
          <li>
            <Link href="/creator/upload-content">Creator upload wizard</Link> — hash
            media locally and build a manifest
          </li>
          <li>
            <Link href="/manifest">Manifest generator</Link> — structured metadata
            for provenance
          </li>
          <li>
            Soroban contracts: verification (<code>stellarproof</code>), certificates (
            <code>provenance</code>), TEE registry (<code>registry</code>)
          </li>
          <li>Testnet deploy via <code>scripts/deploy-testnet.sh</code></li>
        </ul>

        <h2>Phase 2+ (planned)</h2>
        <p>
          Oracle worker, verification APIs, IPFS storage, and AWS Nitro TEE
          attestation. See{" "}
          <a href="https://github.com/tommydebisi/StellarProof/blob/main/docs/ROADMAP.md">
            docs/ROADMAP.md
          </a>
          .
        </p>

        <h2>Contributing</h2>
        <p>
          We participate in the{" "}
          <a href="https://www.drips.network/wave/stellar">Stellar Wave Program</a>.
          See{" "}
          <a href="https://github.com/tommydebisi/StellarProof/blob/main/CONTRIBUTING.md">
            CONTRIBUTING.md
          </a>{" "}
          and open issues tagged <code>wave-candidate</code>.
        </p>
      </main>
    </div>
  );
}
