import { LogOut } from "lucide-react";
import type {
  SidebarData,
  SidebarItem,
} from "@/components/settings-dialog/nav-hook";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { useAuthStore } from "@/store/auth-store";

interface Props {
  currentRoute: SidebarItem;
  data: SidebarData;
  onRouteChange: (r: SidebarItem) => void;
}

export function SettingsSidebar({ data, currentRoute, onRouteChange }: Props) {
  const user = useAuthStore((state) => state.currentUser);
  const logout = useAuthStore((state) => state.logout);

  return (
    <Sidebar className="hidden border-r md:flex" collapsible="none">
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  className="group h-fit cursor-pointer border py-2 transition-colors data-[active=true]:bg-primary data-[active=true]:text-primary-foreground"
                  isActive={currentRoute.name === "Profile"}
                  onClick={() => onRouteChange(data.hidden[0])}
                >
                  <Avatar>
                    <AvatarFallback>
                      {user?.profile.display_name.charAt(0)}
                    </AvatarFallback>
                    <AvatarImage src={user?.profile.avatar_url || undefined} />
                  </Avatar>
                  <div className="flex flex-col">
                    <span>{user?.profile.display_name}</span>
                    <span className="text-muted-foreground text-xs group-hover:text-white group-data-[active=true]:text-white">
                      Edit profile
                    </span>
                  </div>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        {Object.entries(data)
          .filter(([category]) => category !== "hidden")
          .map(([category, items]) => (
            <SidebarGroup className="pt-0" key={category}>
              <SidebarGroupLabel className="font-semibold text-muted-foreground text-xs uppercase">
                {category}
              </SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  {items.map((item) => (
                    <SidebarMenuItem key={item.name}>
                      <SidebarMenuButton
                        className="cursor-pointer data-[active=true]:bg-primary data-[active=true]:text-primary-foreground"
                        isActive={item.name === currentRoute.name}
                        onClick={() => onRouteChange(item)}
                      >
                        <item.icon />
                        {item.name}
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  ))}
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          ))}

        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  className="cursor-pointer text-destructive hover:text-destructive/80"
                  onClick={logout}
                >
                  <LogOut /> Log Out
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
