import { format } from "date-fns";
import { Calendar, Search, Shield } from "lucide-react";
import { Button } from "./components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "./components/ui/card";
import { Input } from "./components/ui/input";
import { useState } from "react";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "./components/ui/popover";
import { Calendar as CalendarComponent } from "@/components/ui/calendar";

function App() {
  const [ipAddress, setIpAddress] = useState("");
  const [date, setDate] = useState<{
    from: Date | undefined;
    to: Date | undefined;
  }>({
    from: undefined,
    to: undefined,
  });
  const [logs, setLogs] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [verifying, setVerifying] = useState<{ [key: string]: boolean }>({});

  const fetchLogs = () => {
    setLoading(true);

    setTimeout(() => {
      const mockLogs = [
        {
          id: "log-1",
          timestamp: "2023-05-06T14:22:31Z",
          source: ipAddress || "192.168.1.105",
          index: "logs-system-2023.05.06",
          document: {
            timestamp: "2023-05-06T14:22:31Z",
            message: "User login successful",
            user_id: "user_12345",
            session_id: "sess_abcdef123456",
            ip_address: "192.168.1.105",
            user_agent:
              "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
            login_type: "password",
            success: true,
            attempt_count: 1,
            location: {
              country: "United States",
              city: "San Francisco",
              coordinates: [-122.4194, 37.7749],
            },
          },
        },
        {
          id: "log-2",
          timestamp: "2023-05-06T14:23:15Z",
          source: ipAddress || "192.168.1.105",
          index: "logs-security-2023.05.06",
          document: {
            timestamp: "2023-05-06T14:23:15Z",
            message: "Failed password attempt",
            user_id: "user_12345",
            session_id: "sess_abcdef123456",
            ip_address: "192.168.1.105",
            user_agent:
              "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
            login_type: "password",
            success: false,
            attempt_count: 3,
            location: {
              country: "United States",
              city: "San Francisco",
              coordinates: [-122.4194, 37.7749],
            },
          },
        },
        {
          id: "log-3",
          timestamp: "2023-05-06T14:25:42Z",
          source: ipAddress || "192.168.1.105",
          index: "logs-system-2023.05.06",
          document: {
            timestamp: "2023-05-06T14:25:42Z",
            message: "Connection timeout after 30s",
            service: "database",
            instance_id: "db-prod-01",
            error_code: "ETIMEDOUT",
            connection_params: {
              host: "db.example.com",
              port: 5432,
              database: "users",
              timeout: 30000,
            },
            retry_count: 2,
            severity: "high",
          },
        },
        {
          id: "log-4",
          timestamp: "2023-05-06T14:30:11Z",
          source: ipAddress || "192.168.1.105",
          index: "logs-performance-2023.05.06",
          document: {
            timestamp: "2023-05-06T14:30:11Z",
            message: "Database query completed in 235ms",
            query_id: "q-987654",
            execution_time: 235,
            rows_returned: 1250,
            cache_hit: false,
            query_type: "SELECT",
            database: "analytics",
            user: "reporting_service",
            client_ip: "192.168.1.105",
            query_hash: "a1b2c3d4e5",
          },
        },
        {
          id: "log-5",
          timestamp: "2023-05-06T14:32:07Z",
          source: ipAddress || "192.168.1.105",
          index: "logs-system-2023.05.06",
          document: {
            timestamp: "2023-05-06T14:32:07Z",
            message: "Cache hit ratio: 78.5%",
            cache_name: "user_profiles",
            hit_count: 12589,
            miss_count: 3452,
            hit_ratio: 0.785,
            eviction_count: 215,
            memory_usage: "1.2GB",
            max_memory: "2GB",
            instance: "cache-01",
          },
        },
      ];

      setLogs(mockLogs);
      setLoading(false);
    }, 1000);
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return format(date, "MM/dd/yyyy");
  };

  const verifyLogIntegrity = (logId: string) => {
    setVerifying((prev) => ({ ...prev, [logId]: true }));

    setTimeout(() => {
      setVerifying((prev) => ({ ...prev, [logId]: false }));
    }, 1500);
  };

  const renderJsonContent = (content: any) => {
    return (
      <pre className="text-xs text-gray-700 overflow-x-auto whitespace-pre-wrap">
        {JSON.stringify(content, null, 2)}
      </pre>
    );
  };

  return (
    <div className="min-h-screen bg-gray-50 text-gray-900">
      <div className="container mx-auto py-8 px-4">
        <Card className="bg-white border-gray-200 shadow-sm">
          <CardHeader>
            <CardTitle className="text-2xl font-bold text-gray-800">
              Audita Logs
            </CardTitle>
            <CardDescription className="text-gray-500">
              Search and verify logs from Elasticsearch
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="flex flex-col md:flex-row gap-4 mb-6">
              <div className="flex-1">
                <div className="relative">
                  <Search className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
                  <Input
                    placeholder="Enter IP address..."
                    value={ipAddress}
                    onChange={(e) => setIpAddress(e.target.value)}
                    className="pl-10 bg-white border-gray-200 text-gray-800 placeholder:text-gray-400 focus-visible:ring-gray-300"
                  />
                </div>
              </div>

              <div className="w-full md:w-auto">
                <Popover>
                  <PopoverTrigger asChild>
                    <Button
                      variant="outline"
                      className="w-full justify-start text-left font-normal bg-white border-gray-200 hover:bg-gray-100 hover:text-gray-900"
                    >
                      <Calendar className="mr-2 h-4 w-4" />
                      {date.from ? (
                        date.to ? (
                          <>
                            {format(date.from, "MM/dd/yyyy")} -{" "}
                            {format(date.to, "MM/dd/yyyy")}
                          </>
                        ) : (
                          format(date.from, "MM/dd/yyyy")
                        )
                      ) : (
                        <span className="text-gray-500">
                          Select a date range
                        </span>
                      )}
                    </Button>
                  </PopoverTrigger>
                  <PopoverContent
                    className="w-auto p-0 bg-white border-gray-200"
                    align="start"
                  >
                    <CalendarComponent
                      initialFocus
                      mode="range"
                      defaultMonth={date.from}
                      selected={date}
                      onSelect={setDate as any}
                      numberOfMonths={2}
                      className="bg-white text-gray-900"
                    />
                  </PopoverContent>
                </Popover>
              </div>

              <Button
                onClick={fetchLogs}
                disabled={loading}
                className="bg-sky-500 hover:bg-sky-600 text-white"
              >
                {loading ? "Searching..." : "Search Logs"}
              </Button>
            </div>

            {logs.length > 0 && (
              <div className="mt-6">
                <h3 className="text-lg font-medium mb-4 text-gray-700">
                  Search Results
                </h3>
                <div className="bg-white rounded-md border border-gray-200 h-[500px] overflow-y-auto p-4 shadow-inner">
                  <div className="space-y-4">
                    {logs.map((log) => (
                      <div
                        key={log.id}
                        className="p-4 bg-white rounded-md border border-gray-200 shadow-sm"
                      >
                        <div className="flex justify-between items-start mb-3">
                          <div className="flex items-center gap-2">
                            <span className="text-sm text-gray-600">
                              Date: {formatDate(log.timestamp)}
                            </span>
                          </div>
                          <Button
                            size="sm"
                            variant="outline"
                            className="h-8 gap-1 bg-white border-gray-200 hover:bg-gray-50 text-gray-700"
                            onClick={() => verifyLogIntegrity(log.id)}
                            disabled={verifying[log.id]}
                          >
                            <Shield className="h-3.5 w-3.5" />
                            {verifying[log.id]
                              ? "Verifying..."
                              : "Verify Integrity"}
                          </Button>
                        </div>
                        <div className="flex flex-col gap-2">
                          <div className="text-xs text-gray-600">
                            <span>
                              Index:{" "}
                              <span className="text-gray-800 font-medium">
                                {log.index}
                              </span>
                            </span>
                          </div>
                          <div className="mt-2 bg-gray-50 p-3 rounded border border-gray-200">
                            {renderJsonContent(log.document)}
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}

export default App;
