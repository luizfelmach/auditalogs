"use client";

import { WagmiProvider as WagProvider } from "wagmi";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { config } from "@/lib/wagmi";

const queryClient = new QueryClient();

export function WagmiProvider({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <WagProvider config={config}>
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    </WagProvider>
  );
}
