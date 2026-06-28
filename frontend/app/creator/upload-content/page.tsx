"use client";

import Header from "@/components/Header";
import { WizardProvider } from "@/context/WizardContext";
import MediaInput from "@/components/wizard/steps/MediaInput";
import ManifestStep from "@/components/wizard/steps/ManifestStep";
import SPVOptions from "@/components/wizard/steps/SPVOptions";
import SPVResults from "@/components/wizard/steps/SPVResults";
import AdvancedInput from "@/components/wizard/steps/AdvancedInput";

export default function UploadContentPage() {
  return (
    <div className="min-h-screen bg-gray-50 dark:bg-[#020617]">
      <Header />
      <main className="mx-auto max-w-4xl px-4 py-10 sm:px-6 lg:px-8">
        <WizardProvider>
          <div className="mb-8 space-y-2">
            <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
              Upload & Verify Content
            </h1>
            <p className="text-gray-600 dark:text-gray-400">
              Hash your media locally, attach a manifest, and prepare provenance
              data for on-chain certification on Stellar.
            </p>
          </div>

          <div className="space-y-8">
            <section className="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-800 dark:bg-gray-900/50">
              <h2 className="mb-4 text-lg font-semibold">1. Upload media</h2>
              <MediaInput />
            </section>

            <section className="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-800 dark:bg-gray-900/50">
              <h2 className="mb-4 text-lg font-semibold">2. Attach manifest</h2>
              <ManifestStep />
            </section>

            <section className="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-800 dark:bg-gray-900/50">
              <h2 className="mb-4 text-lg font-semibold">3. Advanced hashes</h2>
              <AdvancedInput />
            </section>

            <section className="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-800 dark:bg-gray-900/50">
              <h2 className="mb-4 text-lg font-semibold">4. Encryption options</h2>
              <SPVOptions />
            </section>

            <section className="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-800 dark:bg-gray-900/50">
              <h2 className="mb-4 text-lg font-semibold">5. Results</h2>
              <SPVResults />
            </section>
          </div>
        </WizardProvider>
      </main>
    </div>
  );
}
