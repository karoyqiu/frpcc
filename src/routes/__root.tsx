import { createRootRoute, Outlet } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';

import { AppSidebar } from '@/components/app-sidebar';
import { SidebarProvider } from '@/components/ui/sidebar';
import { TooltipProvider } from '@/components/ui/tooltip';

const RootLayout = () => (
  <TooltipProvider>
    <SidebarProvider>
      <AppSidebar />
      <main className="w-full">
        <Outlet />
      </main>
      <TanStackRouterDevtools />
    </SidebarProvider>
  </TooltipProvider>
);

export const Route = createRootRoute({ component: RootLayout });
