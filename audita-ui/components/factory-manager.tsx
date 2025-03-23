"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Loader2, Plus } from "lucide-react";
import { toast } from "sonner";
import { useAccount, useWatchContractEvent, useWriteContract } from "wagmi";
import { FACTORY_ABI } from "@/lib/contract-abis";
import { client } from "@/lib/wagmi";

export function FactoryManager() {
  const { address } = useAccount();
  const [factoryAddress, setFactoryAddress] = useState(
    "0x42699A7612A82f1d9C36148af9C77354759b210b"
  );
  const [isConnected, setIsConnected] = useState(false);
  const [isFetching, setIsFetching] = useState(false);
  const [deployedContracts, setDeployedContracts] = useState<string[]>([]);

  const connectToFactory = async () => {
    if (!factoryAddress) return;
    setIsFetching(true);

    try {
      const result = await client.readContract({
        address: factoryAddress as `0x${string}`,
        abi: FACTORY_ABI,
        functionName: "getDeployedContracts",
        args: [address],
      });
      setDeployedContracts(result as string[]);
      setIsConnected(true);
    } catch (error) {
      toast.error(`Error connecting to factory: ${error}`);
    } finally {
      setIsFetching(false);
    }
  };

  const { writeContract, isPending } = useWriteContract();

  useWatchContractEvent({
    abi: FACTORY_ABI,
    address: factoryAddress as `0x${string}`,
    eventName: "AuditContractCreated",
    async onLogs() {
      const result = await client.readContract({
        address: factoryAddress as `0x${string}`,
        abi: FACTORY_ABI,
        functionName: "getDeployedContracts",
        args: [address],
      });
      console.log(result);
      setDeployedContracts(result as string[]);
    },
  });

  const createNewContract = async () => {
    writeContract({
      address: factoryAddress as `0x${string}`,
      abi: FACTORY_ABI,
      functionName: "createAuditContract",
    });
  };

  const copyToClipboard = (address: string) => {
    navigator.clipboard.writeText(address);
    toast.success(`Address ${address.substring(0, 6)}... copied to clipboard`);
  };

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Factory Management</CardTitle>
          <CardDescription>
            Connect to an AuditabilityFactory contract and manage your audit
            contracts
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4">
            <div className="grid gap-2">
              <Label htmlFor="factory-address">Factory Contract Address</Label>
              <div className="flex gap-2">
                <Input
                  id="factory-address"
                  placeholder="0x..."
                  value={factoryAddress}
                  onChange={(e) => setFactoryAddress(e.target.value)}
                  disabled={isConnected}
                />
                {!isConnected ? (
                  <Button
                    onClick={connectToFactory}
                    disabled={isFetching || !factoryAddress}
                  >
                    {isFetching ? (
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
                      toast.info("Disconnected from factory contract");
                    }}
                  >
                    Disconnect
                  </Button>
                )}
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {isConnected && (
        <Card>
          <CardHeader>
            <CardTitle>Your Audit Contracts</CardTitle>
            <CardDescription>
              Manage your deployed Auditability contracts
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Contract Address</TableHead>
                  <TableHead className="text-right">Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {deployedContracts.map((address, index) => (
                  <TableRow key={index}>
                    <TableCell className="font-mono">{address}</TableCell>
                    <TableCell className="text-right">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => copyToClipboard(address)}
                      >
                        Copy Address
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
          <CardFooter>
            <Button
              onClick={createNewContract}
              disabled={isPending}
              className="w-full"
            >
              {isPending ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  Creating Contract
                </>
              ) : (
                <>
                  <Plus className="mr-2 h-4 w-4" />
                  Create New Audit Contract
                </>
              )}
            </Button>
          </CardFooter>
        </Card>
      )}
    </div>
  );
}
