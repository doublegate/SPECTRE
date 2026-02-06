import { createBrowserRouter, Navigate } from "react-router";
import { MainLayout } from "@/layouts/MainLayout";
import { Dashboard } from "@/pages/Dashboard";
import { Targets } from "@/pages/Targets";
import { Recon } from "@/pages/Recon";
import { Analysis } from "@/pages/Analysis";
import { Comms } from "@/pages/Comms";
import { Campaigns } from "@/pages/Campaigns";
import { CampaignDetail } from "@/pages/CampaignDetail";
import { Reports } from "@/pages/Reports";
import { Settings } from "@/pages/Settings";

export const router = createBrowserRouter([
  {
    element: <MainLayout />,
    children: [
      { index: true, element: <Navigate to="/dashboard" replace /> },
      { path: "dashboard", element: <Dashboard /> },
      { path: "targets", element: <Targets /> },
      { path: "recon", element: <Recon /> },
      { path: "analysis", element: <Analysis /> },
      { path: "comms", element: <Comms /> },
      { path: "campaigns", element: <Campaigns /> },
      { path: "campaigns/:id", element: <CampaignDetail /> },
      { path: "reports", element: <Reports /> },
      { path: "settings", element: <Settings /> },
    ],
  },
]);
