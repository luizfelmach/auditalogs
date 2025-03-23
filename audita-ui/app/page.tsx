"use client";

import { ConnectWallet } from "@/components/connect-wallet";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useAccount, useDisconnect } from "wagmi";
import { FactoryManager } from "@/components/factory-manager";
import { ContractManager } from "@/components/contract-manager";
import { useEffect, useState } from "react";

export default function Home() {
  const { isConnected } = useAccount();
  const { disconnect } = useDisconnect();
  const [hasMounted, setHasMounted] = useState(false);

  useEffect(() => {
    setHasMounted(true);
  }, []);

  if (!hasMounted) {
    return null;
  }

  return (
    <main className="container mx-auto p-4 max-w-6xl">
      <div className="flex flex-col gap-8">
        <div className="flex justify-between items-center">
          <h1 className="text-3xl font-bold">Auditability Contract Manager</h1>
          <div className="flex">
            {isConnected && (
              <Button onClick={() => disconnect()} variant={"destructive"}>
                Disconnect
              </Button>
            )}
            <ConnectWallet />
          </div>
        </div>

        {!isConnected ? (
          <Card>
            <CardHeader>
              <CardTitle>Connect Your Wallet</CardTitle>
              <CardDescription>
                Connect your Ethereum wallet to manage your Auditability
                contracts
              </CardDescription>
            </CardHeader>
            <CardContent>
              <ConnectWallet />
            </CardContent>
          </Card>
        ) : (
          <Tabs defaultValue="factory">
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="factory">Factory Management</TabsTrigger>
              <TabsTrigger value="contract">Contract Management</TabsTrigger>
            </TabsList>
            <TabsContent value="factory">
              <FactoryManager />
            </TabsContent>
            <TabsContent value="contract">
              <ContractManager />
            </TabsContent>
          </Tabs>
        )}
      </div>
    </main>
  );
}
