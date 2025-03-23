"use client";

import { Button } from "@/components/ui/button";
import { Wallet } from "lucide-react";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import { useAccount, useConnect } from "wagmi";
import { metaMask } from "wagmi/connectors";

export function ConnectWallet() {
  const [hasMounted, setHasMounted] = useState(false);
  const { address, isConnected } = useAccount();
  const { connect, error, isPending } = useConnect();

  useEffect(() => {
    if (error) toast.error(`Failed to connect: ${error?.message}`);
    setHasMounted(true);
  }, [error]);

  if (!hasMounted) {
    return null;
  }

  const handleConnect = () => {
    connect({ connector: metaMask() });
  };

  const formatAccount = (address: string) => {
    return `${address.substring(0, 6)}...${address.substring(
      address.length - 4
    )}`;
  };

  return (
    <div>
      {isConnected && address ? (
        <Button variant="outline" className="flex gap-2 items-center">
          <Wallet className="h-4 w-4" />
          <span>{formatAccount(address)}</span>
        </Button>
      ) : (
        <Button
          disabled={isPending}
          onClick={handleConnect}
          className="flex gap-2 items-center"
        >
          <Wallet className="h-4 w-4" />
          <span>Connect Wallet</span>
        </Button>
      )}
    </div>
  );
}
