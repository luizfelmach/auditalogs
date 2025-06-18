import { Activity, Search, Settings, Shield } from "lucide-react";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { Separator } from "@/components/ui/separator";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarRail,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import { useState } from "react";
import { SystemStatusDetailed } from "@/components/system-status-detailed";
import { ElasticsearchSearchAdvanced } from "@/components/elasticsearch-search-advanced";

type PageType = "visao-geral" | "busca-avancada";

const navigationItems = [
  {
    id: "visao-geral" as PageType,
    title: "Visão Geral",
    icon: Activity,
  },
  {
    id: "busca-avancada" as PageType,
    title: "Busca Avançada",
    icon: Search,
  },
];

function AppSidebar({
  currentPage,
  onPageChange,
}: {
  currentPage: PageType;
  onPageChange: (page: PageType) => void;
}) {
  return (
    <Sidebar>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" asChild>
              <div className="flex items-center gap-2 cursor-pointer">
                <div className="flex aspect-square size-8 items-center justify-center rounded-full bg-blue-600 text-white">
                  <Shield className="size-4 text-white" />
                </div>
                <div className="flex flex-col gap-0.5 leading-none">
                  <span className="font-semibold text-blue-900">Audita</span>
                  <span className="text-xs text-blue-700">
                    Sistema de Auditabilidade
                  </span>
                </div>
              </div>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>

      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Navegação</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {navigationItems.map((item) => (
                <SidebarMenuItem key={item.id}>
                  <SidebarMenuButton
                    isActive={currentPage === item.id}
                    onClick={() => onPageChange(item.id)}
                    className={
                      currentPage === item.id ? "bg-blue-100 text-blue-800" : ""
                    }
                  >
                    <item.icon className="size-4" />
                    <span>{item.title}</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter>
        <div className="p-3 bg-gradient-to-r from-blue-50 to-indigo-50 rounded-lg border border-blue-200">
          <div className="flex items-center space-x-2 mb-2">
            <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
            <span className="text-sm font-medium text-blue-900">
              Sistema Online
            </span>
          </div>
          <p className="text-xs text-blue-700">
            Monitoramento ativo de eventos blockchain
          </p>
        </div>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton>
              <Settings className="size-4" />
              <span>Configurações</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>

      <SidebarRail />
    </Sidebar>
  );
}

export function Dashboard() {
  const [currentPage, setCurrentPage] = useState<PageType>("visao-geral");

  const getCurrentPageTitle = () => {
    const page = navigationItems.find((item) => item.id === currentPage);
    return page?.title || "Página";
  };

  return (
    <SidebarProvider>
      <AppSidebar currentPage={currentPage} onPageChange={setCurrentPage} />
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
          <SidebarTrigger className="-ml-1" />
          <Separator orientation="vertical" className="mr-2 h-4" />
          <Breadcrumb>
            <BreadcrumbList>
              <BreadcrumbItem className="hidden md:block">
                <BreadcrumbLink href="#" className="text-blue-600">
                  Audita
                </BreadcrumbLink>
              </BreadcrumbItem>
              <BreadcrumbSeparator className="hidden md:block" />
              <BreadcrumbItem>
                <BreadcrumbPage className="text-blue-800">
                  {getCurrentPageTitle()}
                </BreadcrumbPage>
              </BreadcrumbItem>
            </BreadcrumbList>
          </Breadcrumb>
        </header>

        <div className="flex-1 p-6 bg-gradient-to-br from-blue-50 to-slate-50">
          {currentPage === "visao-geral" && <SystemStatusDetailed />}
          {currentPage === "busca-avancada" && <ElasticsearchSearchAdvanced />}
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
