"use client";

import { useSearchParams } from "next/navigation";
import NavbarResponsive from "@/components/NavbarResponsive";

// Hides the navbar when the page is loaded with ?embed=1 (e.g. iframed on alteredbrainchemistry.com).
export default function SiteChrome({ children }) {
  const searchParams = useSearchParams();
  const isEmbed = searchParams.get("embed") === "1";

  return (
    <div className="site-shell">
      {!isEmbed && <NavbarResponsive />}
      <div className="site-inner-content">
        <main className="site-main">{children}</main>
      </div>
    </div>
  );
}
