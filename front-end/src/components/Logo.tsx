import { cn } from "@/lib/utils";

interface LogoProps {
  className?: string;
  showText?: boolean;
}

/** Logo da marca: pino de localização com "S" + wordmark "SmartLocus". */
export function Logo({ className, showText = true }: LogoProps) {
  return (
    <div className={cn("flex items-center gap-2", className)}>
      <svg viewBox="0 0 48 60" className="h-7 w-auto shrink-0" aria-hidden="true">
        <path
          d="M24 2C12.4 2 3 11.4 3 23c0 14.5 18.4 32.2 19.2 33a2.5 2.5 0 0 0 3.6 0C26.6 55.2 45 37.5 45 23 45 11.4 35.6 2 24 2Z"
          fill="#2563eb"
        />
        <circle cx="24" cy="23" r="14" fill="#fff" />
        <text
          x="24"
          y="31"
          textAnchor="middle"
          fontSize="20"
          fontWeight="700"
          fill="#1e3a8a"
          fontFamily="ui-sans-serif, system-ui, sans-serif"
        >
          S
        </text>
      </svg>
      {showText && (
        <span className="text-lg font-bold tracking-tight">
          <span className="text-[#1e3a8a]">Smart</span>
          <span className="text-primary">Locus</span>
        </span>
      )}
    </div>
  );
}
