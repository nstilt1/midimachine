"use client";

import { useEffect, useRef, useState } from "react";
import Image from "next/image";
import { Download, FileText, Globe, Heart, Home, Menu, ShoppingBag } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetTrigger } from "@/components/ui/sheet";

const SITE = "https://alteredbrainchemistry.com";
export const LOGO_CLICK_EVENT = "midimachine:logo-click";

const primaryRoutes = [
  { name: "Home", href: `${SITE}/`, icon: Home },
  { name: "Shop", href: `${SITE}/shop`, icon: ShoppingBag },
];

const resourcesMenuItems = [
  { name: "Blog", href: `${SITE}/blog`, icon: FileText },
  { name: "Downloads", href: `${SITE}/downloads`, icon: Download },
  { name: "Web Apps", href: `${SITE}/webapps`, icon: Globe },
];

const authRoutes = [
  { name: "Sign In", href: `${SITE}/login` },
  { name: "Sign Up", href: `${SITE}/register` },
];

function Separator({ className = "" }) {
  return <div role="separator" className={`h-px w-full bg-border ${className}`} />;
}

export default function NavbarResponsive() {
  const [mobileOpen, setMobileOpen] = useState(false);
  const [resourcesOpen, setResourcesOpen] = useState(false);
  const [navVisible, setNavVisible] = useState(true);
  const lastScrollY = useRef(0);
  const resourcesMenuRef = useRef(null);

  useEffect(() => {
    function onPointerDown(event) {
      if (resourcesMenuRef.current && !resourcesMenuRef.current.contains(event.target)) {
        setResourcesOpen(false);
      }
    }
    document.addEventListener("mousedown", onPointerDown);
    return () => document.removeEventListener("mousedown", onPointerDown);
  }, []);

  useEffect(() => {
    lastScrollY.current = window.scrollY;
    function handleScroll() {
      const y = window.scrollY;
      if (y < 16 || y < lastScrollY.current) setNavVisible(true);
      else if (y > lastScrollY.current) setNavVisible(false);
      lastScrollY.current = y;
    }
    window.addEventListener("scroll", handleScroll, { passive: true });
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  // The logo still toggles the Midi Machine's advanced controls.
  const onLogoClick = () => window.dispatchEvent(new Event(LOGO_CLICK_EVENT));

  return (
    <nav
      className={`sticky top-0 z-50 overflow-visible border-b border-border/60 bg-background/90 backdrop-blur transition-transform duration-300 ${
        navVisible ? "translate-y-0" : "-translate-y-full"
      }`}
    >
      <div className="mx-auto flex h-20 w-full max-w-7xl items-center overflow-visible px-6 md:px-8">
        <button
          type="button"
          onClick={onLogoClick}
          className="group inline-flex shrink-0 items-center rounded-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          aria-label="Toggle advanced controls"
        >
          <div className="relative h-12 w-[200px] overflow-hidden sm:h-14 sm:w-[240px] md:h-16 md:w-[300px]">
            <Image
              src="/altered-brain-chemistry-logo.png"
              alt="Altered Brain Chemistry logo"
              fill
              sizes="(max-width: 640px) 200px, (max-width: 768px) 240px, 300px"
              className="object-contain object-left abc-logo-cycle"
              priority
            />
          </div>
        </button>

        <div className="ml-auto flex items-center gap-2">
          <div className="hidden items-center gap-1 md:flex">
            {primaryRoutes.map(({ name, href, icon: Icon }) => (
              <a key={href} href={href}>
                <Button variant="ghost" className="nav-btn gap-2">
                  <Icon className="h-4 w-4" />
                  {name}
                </Button>
              </a>
            ))}

            <div
              ref={resourcesMenuRef}
              className="relative"
              onMouseEnter={() => setResourcesOpen(true)}
              onMouseLeave={() => setResourcesOpen(false)}
            >
              <Button
                type="button"
                variant="ghost"
                className="nav-btn gap-2"
                aria-expanded={resourcesOpen}
                onClick={() => setResourcesOpen((v) => !v)}
              >
                Content
              </Button>

              {resourcesOpen ? (
                <>
                  <div className="absolute left-0 top-full z-[9998] h-2 w-full" />
                  <div className="absolute left-1/2 top-full z-[9999] mt-2 w-64 -translate-x-1/2 rounded-2xl border border-border/60 bg-popover p-2 shadow-xl">
                    <div className="px-3 py-2">
                      <div className="text-sm font-medium text-popover-foreground">Content</div>
                      <div className="text-xs text-muted-foreground">Helpful links and downloads</div>
                    </div>
                    <Separator className="my-1" />
                    <div className="flex flex-col">
                      {resourcesMenuItems.map(({ name, href, icon: Icon }) => (
                        <a
                          key={href}
                          href={href}
                          className="rounded-xl px-3 py-2 text-sm text-popover-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
                        >
                          <span className="inline-flex items-center gap-2">
                            <Icon className="h-4 w-4" />
                            {name}
                          </span>
                        </a>
                      ))}
                    </div>
                  </div>
                </>
              ) : null}
            </div>

            {authRoutes.map(({ name, href }) => (
              <a key={href} href={href}>
                <Button variant="ghost" className="nav-btn gap-2">{name}</Button>
              </a>
            ))}
          </div>

          <a href={`${SITE}/support-us`} className="hidden md:block">
            <Button className="gap-2 rounded-xl">
              <Heart className="h-4 w-4" />
              Support Us
            </Button>
          </a>

          <div className="md:hidden">
            <Sheet open={mobileOpen} onOpenChange={setMobileOpen}>
              <SheetTrigger asChild>
                <Button variant="ghost" size="icon" aria-label="Open navigation">
                  <Menu className="h-5 w-5" />
                </Button>
              </SheetTrigger>

              <SheetContent side="right" className="w-[88vw] max-w-sm">
                <SheetHeader>
                  <SheetTitle className="text-left">Navigation</SheetTitle>
                </SheetHeader>

                <div className="mt-6 flex flex-col gap-1 px-1">
                  <a href={`${SITE}/support-us`}>
                    <Button className="w-full justify-start gap-2 rounded-xl">
                      <Heart className="h-4 w-4" />
                      Support Us
                    </Button>
                  </a>
                  {primaryRoutes.map(({ name, href, icon: Icon }) => (
                    <a key={href} href={href}>
                      <Button variant="ghost" className="w-full justify-start gap-2">
                        <Icon className="h-4 w-4" />
                        {name}
                      </Button>
                    </a>
                  ))}
                </div>

                <Separator className="my-6" />

                <div className="mb-2 px-3">
                  <div className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">Content</div>
                  <div className="mt-0.5 text-xs text-muted-foreground/70">Helpful links and downloads</div>
                </div>
                <div className="flex flex-col gap-1 px-1">
                  {resourcesMenuItems.map(({ name, href, icon: Icon }) => (
                    <a key={href} href={href}>
                      <Button variant="ghost" className="w-full justify-start gap-2">
                        <Icon className="h-4 w-4" />
                        {name}
                      </Button>
                    </a>
                  ))}
                </div>

                <Separator className="my-6" />

                <div className="flex flex-col gap-1 px-1">
                  {authRoutes.map(({ name, href }) => (
                    <a key={href} href={href}>
                      <Button variant="ghost" className="w-full justify-start gap-2">{name}</Button>
                    </a>
                  ))}
                </div>
              </SheetContent>
            </Sheet>
          </div>
        </div>
      </div>
    </nav>
  );
}
