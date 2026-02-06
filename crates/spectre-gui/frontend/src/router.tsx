import { lazy, Suspense } from "react";
import { createBrowserRouter, Navigate } from "react-router";
import { MainLayout } from "@/layouts/MainLayout";

// Lazy load page components for code splitting
const Dashboard = lazy(() => import("@/pages/Dashboard"));
const Targets = lazy(() => import("@/pages/Targets"));
const Recon = lazy(() => import("@/pages/Recon"));
const Analysis = lazy(() => import("@/pages/Analysis"));
const Comms = lazy(() => import("@/pages/Comms"));
const Campaigns = lazy(() => import("@/pages/Campaigns"));
const CampaignDetail = lazy(() => import("@/pages/CampaignDetail"));
const Reports = lazy(() => import("@/pages/Reports"));
const Settings = lazy(() => import("@/pages/Settings"));

// Loading fallback component
function LoadingFallback() {
  return (
    <div className="flex items-center justify-center h-screen">
      <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
    </div>
  );
}

export const router = createBrowserRouter([
  {
    element: <MainLayout />,
    children: [
      { index: true, element: <Navigate to="/dashboard" replace /> },
      {
        path: "dashboard",
        element: (
          <Suspense fallback={<LoadingFallback />}>
            <Dashboard />
          </Suspense>
        ),
      },
      {
        path: "targets",
        element: (
          <Suspense fallback={<LoadingFallback />}>
            <Targets />
          </Suspense>
        ),
      },
      {
        path: "recon",
        element: (
          <Suspense fallback={<LoadingFallback />}>
            <Recon />
          </Suspense>
        ),
      },
      {
        path: "analysis",
        element: (
          <Suspense fallback={<LoadingFallback />}>
            <Analysis />
          </Suspense>
        ),
      },
      {
        path: "comms",
        element: (
          <Suspense fallback={<LoadingFallback />}>
            <Comms />
          </Suspense>
        ),
      },
      {
        path: "campaigns",
        element: (
          <Suspense fallback={<LoadingFallback />}>
            <Campaigns />
          </Suspense>
        ),
      },
      {
        path: "campaigns/:id",
        element: (
          <Suspense fallback={<LoadingFallback />}>
            <CampaignDetail />
          </Suspense>
        ),
      },
      {
        path: "reports",
        element: (
          <Suspense fallback={<LoadingFallback />}>
            <Reports />
          </Suspense>
        ),
      },
      {
        path: "settings",
        element: (
          <Suspense fallback={<LoadingFallback />}>
            <Settings />
          </Suspense>
        ),
      },
    ],
  },
]);
