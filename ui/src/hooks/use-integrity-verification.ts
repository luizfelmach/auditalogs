import { useState } from "react";
import type {
  VerificationResult,
  ElasticsearchDocument,
} from "../types/search";
import { retrieveElasticHash, retrieveEthereumHash } from "@/lib/api";

export const useIntegrityVerification = () => {
  const [verificationResults, setVerificationResults] = useState<
    VerificationResult[]
  >([]);
  const [isVerifying, setIsVerifying] = useState(false);

  const verifyIntegrity = async (documents: ElasticsearchDocument[]) => {
    if (documents.length === 0) return;

    setIsVerifying(true);

    // Initialize all results with loading state
    const initialResults: VerificationResult[] = documents.map((doc) => ({
      documentId: doc.id,
      hashElastic: null,
      hashEthereumLoading: true,
      hashElasticLoading: true,
      hashEthereum: null,
      isIntact: null,
      verifiedAt: null,
    }));

    setVerificationResults(initialResults);

    for (let i = 0; i < documents.length; i++) {
      const doc = documents[i];
      const hashEthereum = await retrieveEthereumHash(doc.index);
      setVerificationResults((prev) =>
        prev.map((result) =>
          result.documentId === doc.id
            ? {
                ...result,
                hashEthereum,
                hashEthereumLoading: false,
              }
            : result,
        ),
      );
    }
    for (let i = 0; i < documents.length; i++) {
      const doc = documents[i];
      const hashElastic = await retrieveElasticHash(doc.index);
      setVerificationResults((prev) =>
        prev.map((result) =>
          result.documentId === doc.id
            ? {
                ...result,
                hashElastic,
                hashElasticLoading: false,
                isIntact: hashElastic === result.hashEthereum,
                verifiedAt: new Date().toISOString(),
              }
            : result,
        ),
      );
      if (i === documents.length - 1) {
        setTimeout(() => setIsVerifying(false), 500);
      }
    }

    // // Simulate verification delay (different for each document)
    // const delay = Math.random() * 10000 + 5000; // 1-3 seconds

    // setTimeout(() => {
    //   const hashElastic = `0x${Math.random().toString(16).substr(2, 64)}`;
    //   const hashEthereum =
    //     Math.random() > 0.4
    //       ? `0x${Math.random().toString(16).substr(2, 64)}`
    //       : null;
    //   const isIntact = hashEthereum ? Math.random() > 0.3 : null;

    //   setVerificationResults((prev) =>
    //     prev.map((result) =>
    //       result.documentId === doc.id
    //         ? {
    //             ...result,
    //             hashElastic,
    //             hashEthereum,
    //             isIntact,
    //             verifiedAt: new Date().toISOString(),
    //             isLoading: false,
    //           }
    //         : result,
    //     ),
    //   );

    //   // Check if this is the last document
    //   if (i === documents.length - 1) {
    //     setTimeout(() => setIsVerifying(false), 500);
    //   }
    // }, delay);
  };

  return {
    verificationResults,
    isVerifying,
    verifyIntegrity,
  };
};
