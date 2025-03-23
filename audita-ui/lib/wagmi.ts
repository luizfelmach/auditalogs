import { createPublicClient } from "viem";
import { http, createConfig } from "wagmi";
import { hardhat } from "wagmi/chains";
import { metaMask } from "wagmi/connectors";

export const client = createPublicClient({
  chain: hardhat,
  transport: http(),
});

export const config = createConfig({
  chains: [hardhat],
  connectors: [metaMask()],
  transports: {
    [hardhat.id]: http(),
  },
});
