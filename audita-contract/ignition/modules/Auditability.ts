import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const AuditabilityFactoryModule = buildModule("AuditabilityFactoryModule", (m) => {
  const auditability = m.contract("AuditabilityFactory");
  return { auditability };
});

export default AuditabilityFactoryModule;
