import { useState, useEffect } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Progress } from "@/components/ui/progress";
import { Button } from "@/components/ui/button";
import {
  CheckCircle,
  AlertCircle,
  XCircle,
  Activity,
  Database,
  Globe,
  FileCode,
  Clock,
  Hash,
  TrendingUp,
  Server,
  Zap,
  RefreshCw,
} from "lucide-react";

interface ContractEvent {
  id: number;
  index: number;
  hash: string;
  timestamp: string;
  type: string;
  gasUsed: number;
  blockNumber: number;
}

export function SystemStatusDetailed() {
  const [events, setEvents] = useState<ContractEvent[]>([]);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [lastUpdate, setLastUpdate] = useState(new Date());

  // Simular eventos em tempo real
  useEffect(() => {
    const generateMockEvents = () => {
      const eventTypes = [
        "LogBatchAdded",
        "OwnershipTransferred",
        "ContractUpgraded",
        "BatchProcessed",
      ];
      const newEvents: ContractEvent[] = [];

      for (let i = 0; i < 8; i++) {
        newEvents.push({
          id: Date.now() + i,
          index: 1250 + i,
          hash: `0x${Math.random().toString(16).substr(2, 64)}`,
          timestamp: new Date(Date.now() - i * 300000).toISOString(), // 5 min intervals
          type: eventTypes[Math.floor(Math.random() * eventTypes.length)],
          gasUsed: Math.floor(Math.random() * 50000) + 21000,
          blockNumber: 18500000 + Math.floor(Math.random() * 1000),
        });
      }

      setEvents(
        newEvents.sort(
          (a, b) =>
            new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime(),
        ),
      );
    };

    generateMockEvents();

    // Atualizar a cada 30 segundos
    const interval = setInterval(() => {
      generateMockEvents();
      setLastUpdate(new Date());
    }, 30000);

    return () => clearInterval(interval);
  }, []);

  const refreshEvents = async () => {
    setIsRefreshing(true);
    // Simular delay de refresh
    setTimeout(() => {
      const newEvent: ContractEvent = {
        id: Date.now(),
        index:
          events.length > 0
            ? Math.max(...events.map((e) => e.index)) + 1
            : 1251,
        hash: `0x${Math.random().toString(16).substr(2, 64)}`,
        timestamp: new Date().toISOString(),
        type: "LogBatchAdded",
        gasUsed: Math.floor(Math.random() * 50000) + 21000,
        blockNumber: 18500000 + Math.floor(Math.random() * 1000),
      };

      setEvents((prev) => [newEvent, ...prev.slice(0, 7)]);
      setLastUpdate(new Date());
      setIsRefreshing(false);
    }, 1500);
  };

  const connectionStatus = [
    {
      name: "Nó Ethereum",
      status: "connected",
      icon: Globe,
      color: "text-green-600",
      connection: "Mainnet via Infura",
      detail: "https://mainnet.infura.io/v3/***",
    },
    {
      name: "Smart Contract",
      status: "connected",
      icon: FileCode,
      color: "text-green-600",
      connection: "0x742d35...4C4C4C4C",
      detail: "AuditLog v1.2.0",
    },
    {
      name: "Elasticsearch",
      status: "connected",
      icon: Database,
      color: "text-green-600",
      connection: "audit-cluster-prod",
      detail: "https://es-cluster.audita.com:9200",
    },
    {
      name: "API Gateway",
      status: "connected",
      icon: Server,
      color: "text-green-600",
      connection: "api.audita.com",
      detail: "Load Balancer - 3 instâncias",
    },
  ];

  const systemMetrics = {
    totalTransactions: 15847,
    totalBatches: 1251,
    avgGasPrice: "25.4 gwei",
    networkHashrate: "245.7 TH/s",
    activeNodes: 3,
    dataIndexed: "2.4 TB",
    avgResponseTime: "120ms",
    successRate: 99.7,
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "connected":
        return <CheckCircle className="h-5 w-5 text-green-600" />;
      case "warning":
        return <AlertCircle className="h-5 w-5 text-yellow-600" />;
      case "error":
        return <XCircle className="h-5 w-5 text-red-600" />;
      default:
        return <AlertCircle className="h-5 w-5 text-gray-400" />;
    }
  };

  const getEventTypeBadge = (type: string) => {
    switch (type) {
      case "LogBatchAdded":
        return (
          <Badge className="bg-blue-100 text-blue-800">Batch Adicionado</Badge>
        );
      case "OwnershipTransferred":
        return (
          <Badge className="bg-purple-100 text-purple-800">Transferência</Badge>
        );
      case "ContractUpgraded":
        return <Badge className="bg-green-100 text-green-800">Upgrade</Badge>;
      case "BatchProcessed":
        return (
          <Badge className="bg-orange-100 text-orange-800">Processado</Badge>
        );
      default:
        return <Badge variant="outline">{type}</Badge>;
    }
  };

  const truncateHash = (hash: string) => {
    return `${hash.slice(0, 10)}...${hash.slice(-8)}`;
  };

  return (
    <div className="space-y-6">
      {/* Header com status geral */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card>
          <CardContent className="p-4">
            <div className="flex items-center space-x-2">
              <div className="p-2 bg-green-100 rounded-full">
                <CheckCircle className="h-5 w-5 text-green-600" />
              </div>
              <div>
                <p className="text-sm font-medium">Status Geral</p>
                <p className="text-lg font-bold text-green-600">Online</p>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center space-x-2">
              <div className="p-2 bg-blue-100 rounded-full">
                <Activity className="h-5 w-5 text-blue-600" />
              </div>
              <div>
                <p className="text-sm font-medium">Uptime</p>
                <p className="text-lg font-bold text-blue-600">99.9%</p>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center space-x-2">
              <div className="p-2 bg-purple-100 rounded-full">
                <TrendingUp className="h-5 w-5 text-purple-600" />
              </div>
              <div>
                <p className="text-sm font-medium">Performance</p>
                <p className="text-lg font-bold text-purple-600">Excelente</p>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center space-x-2">
              <div className="p-2 bg-orange-100 rounded-full">
                <Zap className="h-5 w-5 text-orange-600" />
              </div>
              <div>
                <p className="text-sm font-medium">Última Sync</p>
                <p className="text-lg font-bold text-orange-600">Agora</p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Status das Conexões */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <Activity className="h-5 w-5 text-blue-600" />
            <span>Status das Conexões</span>
          </CardTitle>
          <CardDescription>
            Monitoramento em tempo real das conexões do sistema
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            {connectionStatus.map((connection) => {
              const Icon = connection.icon;
              return (
                <div
                  key={connection.name}
                  className="flex flex-col space-y-3 p-4 border rounded-lg bg-gradient-to-r from-green-50 to-green-100"
                >
                  <div className="flex items-center space-x-3">
                    <Icon className={`h-6 w-6 ${connection.color}`} />
                    <div className="flex-1">
                      <p className="font-medium text-sm">{connection.name}</p>
                      <div className="flex items-center space-x-2 mt-1">
                        {getStatusIcon(connection.status)}
                        <Badge
                          variant="default"
                          className="bg-green-600 text-xs"
                        >
                          Online
                        </Badge>
                      </div>
                    </div>
                  </div>

                  <div className="space-y-1">
                    <p className="text-sm font-medium text-gray-900">
                      {connection.connection}
                    </p>
                    <p
                      className="text-xs text-muted-foreground truncate"
                      title={connection.detail}
                    >
                      {connection.detail}
                    </p>
                  </div>
                </div>
              );
            })}
          </div>
        </CardContent>
      </Card>

      {/* Métricas do Sistema */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <FileCode className="h-5 w-5 text-blue-600" />
              <span>Métricas Blockchain</span>
            </CardTitle>
            <CardDescription>
              Estatísticas da blockchain e smart contract
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div className="text-center p-3 bg-blue-50 rounded-lg">
                <p className="text-2xl font-bold text-blue-600">
                  {systemMetrics.totalTransactions.toLocaleString()}
                </p>
                <p className="text-sm text-muted-foreground">
                  Total de Transações
                </p>
              </div>
              <div className="text-center p-3 bg-indigo-50 rounded-lg">
                <p className="text-2xl font-bold text-indigo-600">
                  {systemMetrics.totalBatches.toLocaleString()}
                </p>
                <p className="text-sm text-muted-foreground">
                  Batches Processados
                </p>
              </div>
            </div>

            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-sm font-medium">Gas Price Médio</span>
                <span className="text-sm font-bold text-blue-600">
                  {systemMetrics.avgGasPrice}
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm font-medium">Network Hashrate</span>
                <span className="text-sm font-bold text-blue-600">
                  {systemMetrics.networkHashrate}
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm font-medium">Taxa de Sucesso</span>
                <span className="text-sm font-bold text-green-600">
                  {systemMetrics.successRate}%
                </span>
              </div>
            </div>

            <div>
              <div className="flex justify-between text-sm mb-2">
                <span>Performance da Rede</span>
                <span>{systemMetrics.successRate}%</span>
              </div>
              <Progress value={systemMetrics.successRate} className="h-2" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <Database className="h-5 w-5 text-blue-600" />
              <span>Métricas do Sistema</span>
            </CardTitle>
            <CardDescription>
              Performance e utilização dos recursos
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div className="text-center p-3 bg-green-50 rounded-lg">
                <p className="text-2xl font-bold text-green-600">
                  {systemMetrics.activeNodes}
                </p>
                <p className="text-sm text-muted-foreground">Nós Ativos</p>
              </div>
              <div className="text-center p-3 bg-purple-50 rounded-lg">
                <p className="text-2xl font-bold text-purple-600">
                  {systemMetrics.dataIndexed}
                </p>
                <p className="text-sm text-muted-foreground">Dados Indexados</p>
              </div>
            </div>

            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-sm font-medium">Tempo de Resposta</span>
                <span className="text-sm font-bold text-green-600">
                  {systemMetrics.avgResponseTime}
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm font-medium">Última Atualização</span>
                <span className="text-sm text-muted-foreground">
                  {lastUpdate.toLocaleTimeString()}
                </span>
              </div>
            </div>

            <div>
              <div className="flex justify-between text-sm mb-2">
                <span>Capacidade de Processamento</span>
                <span>87%</span>
              </div>
              <Progress value={87} className="h-2" />
            </div>

            <div>
              <div className="flex justify-between text-sm mb-2">
                <span>Utilização de Memória</span>
                <span>64%</span>
              </div>
              <Progress value={64} className="h-2" />
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Eventos em Tempo Real */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle className="flex items-center space-x-2">
                <Hash className="h-5 w-5 text-blue-600" />
                <span>Eventos do Contrato em Tempo Real</span>
              </CardTitle>
              <CardDescription>
                Últimos eventos registrados no smart contract
              </CardDescription>
            </div>
            <Button
              variant="outline"
              size="sm"
              onClick={refreshEvents}
              disabled={isRefreshing}
              className="flex items-center space-x-2"
            >
              <RefreshCw
                className={`h-4 w-4 ${isRefreshing ? "animate-spin" : ""}`}
              />
              <span>Atualizar</span>
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            {events.map((event, index) => (
              <div
                key={event.id}
                className={`flex items-center justify-between p-4 border rounded-lg transition-all duration-300 ${
                  index === 0 ? "bg-blue-50 border-blue-200" : "bg-gray-50"
                }`}
              >
                <div className="flex items-center space-x-4">
                  <div className="flex flex-col items-center">
                    <div className="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
                      <span className="text-xs font-bold text-blue-600">
                        #{event.index}
                      </span>
                    </div>
                    <div className="text-xs text-muted-foreground mt-1">
                      Índice
                    </div>
                  </div>

                  <div className="flex-1">
                    <div className="flex items-center space-x-2 mb-1">
                      {getEventTypeBadge(event.type)}
                      <Badge variant="outline" className="text-xs">
                        Block {event.blockNumber.toLocaleString()}
                      </Badge>
                    </div>
                    <p className="font-mono text-sm text-gray-600">
                      {truncateHash(event.hash)}
                    </p>
                    <div className="flex items-center space-x-4 mt-2 text-xs text-muted-foreground">
                      <span className="flex items-center space-x-1">
                        <Clock className="h-3 w-3" />
                        <span>
                          {new Date(event.timestamp).toLocaleString()}
                        </span>
                      </span>
                      <span>Gas: {event.gasUsed.toLocaleString()}</span>
                    </div>
                  </div>
                </div>

                <div className="text-right">
                  <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                  <div className="text-xs text-muted-foreground mt-1">Live</div>
                </div>
              </div>
            ))}
          </div>

          <div className="mt-4 p-3 bg-blue-50 border border-blue-200 rounded-lg">
            <div className="flex items-center space-x-2">
              <Activity className="h-4 w-4 text-blue-600" />
              <span className="text-sm text-blue-800">
                Sistema monitorando eventos em tempo real. Última atualização:{" "}
                {lastUpdate.toLocaleTimeString()}
              </span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
