import { Badge } from "@/components/ui/badge";
import { isCritico, type Item } from "@/types";

export function StatusBadge({ item }: { item: Item }) {
  return isCritico(item) ? (
    <Badge variant="destructive">CRÍTICO</Badge>
  ) : (
    <Badge variant="success">OK</Badge>
  );
}
