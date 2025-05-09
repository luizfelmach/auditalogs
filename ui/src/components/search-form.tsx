import type React from "react";

import { useState } from "react";
import { Calendar } from "@/components/ui/calendar";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { Card, CardContent } from "@/components/ui/card";
import { CalendarIcon, Search } from "lucide-react";
import { format } from "date-fns";
import { cn } from "@/lib/utils";
import type { SearchParams } from "@/types/search";

interface SearchFormProps {
  onSearch: (params: SearchParams) => void;
}

export function SearchForm({ onSearch }: SearchFormProps) {
  const [ipAddress, setIpAddress] = useState("");
  const [dateRange, setDateRange] = useState<{
    from: Date | undefined;
    to: Date | undefined;
  }>({
    from: undefined,
    to: undefined,
  });
  const [isFromCalendarOpen, setIsFromCalendarOpen] = useState(false);
  const [isToCalendarOpen, setIsToCalendarOpen] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSearch({
      ipAddress,
      dateFrom: dateRange.from,
      dateTo: dateRange.to,
    });
  };

  return (
    <Card className="border-[#00BFB3]/20 shadow-md">
      <CardContent className="pt-6 max-w-2xl">
        <form onSubmit={handleSubmit} className="space-y-6">
          <div className="space-y-2">
            <Label htmlFor="ip-address" className="text-sm font-medium">
              IP Address
            </Label>
            <div className="relative">
              <Search className="absolute left-3 top-2.5 h-4 w-4 text-[#00BFB3]" />
              <Input
                id="ip-address"
                placeholder="Enter IP address to search"
                value={ipAddress}
                onChange={(e) => setIpAddress(e.target.value)}
                className="pl-10 border-slate-300 focus-visible:ring-[#00BFB3]"
              />
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label className="text-sm font-medium">From Date</Label>
              <Popover
                open={isFromCalendarOpen}
                onOpenChange={setIsFromCalendarOpen}
              >
                <PopoverTrigger asChild>
                  <Button
                    variant="outline"
                    className={cn(
                      "w-full justify-start text-left font-normal border-slate-300",
                      !dateRange.from && "text-muted-foreground",
                    )}
                  >
                    <CalendarIcon className="mr-2 h-4 w-4 text-[#00BFB3]" />
                    {dateRange.from
                      ? format(dateRange.from, "PPP")
                      : "Select date"}
                  </Button>
                </PopoverTrigger>
                <PopoverContent className="w-auto p-0">
                  <Calendar
                    mode="single"
                    selected={dateRange.from}
                    onSelect={(date) => {
                      setDateRange((prev) => ({ ...prev, from: date }));
                      setIsFromCalendarOpen(false);
                    }}
                    initialFocus
                  />
                </PopoverContent>
              </Popover>
            </div>

            <div className="space-y-2">
              <Label className="text-sm font-medium">To Date</Label>
              <Popover
                open={isToCalendarOpen}
                onOpenChange={setIsToCalendarOpen}
              >
                <PopoverTrigger asChild>
                  <Button
                    variant="outline"
                    className={cn(
                      "w-full justify-start text-left font-normal border-slate-300",
                      !dateRange.to && "text-muted-foreground",
                    )}
                  >
                    <CalendarIcon className="mr-2 h-4 w-4 text-[#00BFB3]" />
                    {dateRange.to ? format(dateRange.to, "PPP") : "Select date"}
                  </Button>
                </PopoverTrigger>
                <PopoverContent className="w-auto p-0">
                  <Calendar
                    mode="single"
                    selected={dateRange.to}
                    onSelect={(date) => {
                      setDateRange((prev) => ({ ...prev, to: date }));
                      setIsToCalendarOpen(false);
                    }}
                    initialFocus
                  />
                </PopoverContent>
              </Popover>
            </div>
          </div>

          <Button
            type="submit"
            className="bg-gradient-to-r from-[#00BFB3] to-[#0077CC] hover:from-[#00A5A5] hover:to-[#0066B3]"
          >
            Search Documents
          </Button>
        </form>
      </CardContent>
    </Card>
  );
}
