"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Loader2, Check, X } from "lucide-react";
import { toast } from "sonner";
import { client } from "@/lib/wagmi";
import { AUDITABILITY_ABI } from "@/lib/contract-abis";
import { useAccount } from "wagmi";

export function ContractManager() {
  const { address } = useAccount();
  const [contractAddress, setContractAddress] = useState("");
  const [isConnected, setIsConnected] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [contractOwner, setContractOwner] = useState<string | null>();

  const [verifyIndex, setVerifyIndex] = useState("");
  const [verifyHash, setVerifyHash] = useState("");
  const [isVerifying, setIsVerifying] = useState(false);

  const connectToContract = async () => {
    if (!contractAddress) return;
    setIsLoading(true);

    try {
      const result = await client.readContract({
        address: contractAddress as `0x${string}`,
        abi: AUDITABILITY_ABI,
        functionName: "owner",
        args: [],
      });
      setContractOwner(result as string);
      setIsConnected(true);
    } catch (error) {
      toast.error(`Error connecting to contract: ${error}`);
    } finally {
      setIsLoading(false);
    }
  };

  const verifyProof = async () => {
    if (!verifyIndex || !verifyHash) return;

    setIsVerifying(true);

    try {
      const result = await client.readContract({
        address: contractAddress as `0x${string}`,
        abi: AUDITABILITY_ABI,
        functionName: "proof",
        args: [verifyIndex, verifyHash],
      });

      if (result) toast.success("Verification successful");
      else toast.error("Verification failed");
    } catch (error) {
      toast.error(`Verification failed: ${error}`);
    } finally {
      setIsVerifying(false);
    }
  };

  const isOwner = address?.toLowerCase() === contractOwner?.toLowerCase();

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Contract Management</CardTitle>
          <CardDescription>
            Connect to an Auditability contract to store and verify data
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4">
            <div className="grid gap-2">
              <Label htmlFor="contract-address">Audit Contract Address</Label>
              <div className="flex gap-2">
                <Input
                  id="contract-address"
                  placeholder="0x..."
                  value={contractAddress}
                  onChange={(e) => setContractAddress(e.target.value)}
                  disabled={isConnected}
                />
                {!isConnected ? (
                  <Button
                    onClick={connectToContract}
                    disabled={isLoading || !contractAddress}
                  >
                    {isLoading ? (
                      <>
                        <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                        Connecting
                      </>
                    ) : (
                      "Connect"
                    )}
                  </Button>
                ) : (
                  <Button
                    variant="outline"
                    onClick={() => {
                      setIsConnected(false);
                      toast.info("Disconnected from audit contract");
                    }}
                  >
                    Disconnect
                  </Button>
                )}
              </div>
            </div>

            {isConnected && (
              <div className="pt-2">
                <div className="flex items-center gap-2 text-sm">
                  <span className="font-medium">Contract Owner:</span>
                  <span className="font-mono">{contractOwner}</span>
                </div>
                <div className="flex items-center gap-2 text-sm mt-1">
                  <span className="font-medium">You are the owner:</span>
                  {isOwner ? (
                    <Check className="h-4 w-4 text-green-500" />
                  ) : (
                    <X className="h-4 w-4 text-red-500" />
                  )}
                </div>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {isConnected && (
        <Card>
          <CardHeader>
            <CardTitle>Manage Audit Data</CardTitle>
            <CardDescription>
              Store, verify, and check audit data
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4 py-4">
              <div className="grid gap-2">
                <Label htmlFor="verify-index">Index</Label>
                <Input
                  id="verify-index"
                  placeholder="Enter index to verify"
                  value={verifyIndex}
                  onChange={(e) => setVerifyIndex(e.target.value)}
                />
              </div>
              <div className="grid gap-2">
                <Label htmlFor="verify-hash">Hash Value</Label>
                <Input
                  id="verify-hash"
                  placeholder="Enter hash or text to verify"
                  value={verifyHash}
                  onChange={(e) => setVerifyHash(e.target.value)}
                />
                <p className="text-xs text-muted-foreground">
                  Enter a hex hash (0x...) or plain text that will be hashed
                </p>
              </div>
              <Button
                onClick={verifyProof}
                disabled={isVerifying || !verifyIndex || !verifyHash}
                className="w-full"
              >
                {isVerifying ? (
                  <>
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    Verifying
                  </>
                ) : (
                  "Verify Proof"
                )}
              </Button>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
