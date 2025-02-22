import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const AuditabilityModule = buildModule("AuditabilityModule", (m) => {
  const auditability = m.contract("Auditability");
  return { auditability };
});

export default AuditabilityModule;
