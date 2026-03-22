import { useCallback } from "react";
import type { GroupInfoDto } from "./types";

interface GroupHeaderProps {
  group: GroupInfoDto;
  onToggle: (groupValue: string | null) => void;
}

export function GroupHeader({ group, onToggle }: GroupHeaderProps) {
  const handleClick = useCallback(() => {
    onToggle(group.value);
  }, [group.value, onToggle]);

  return (
    <button
      type="button"
      className="flex items-center gap-2 w-full px-2.5 py-1.5 border-none border-b border-border bg-muted cursor-pointer font-inherit text-sm text-left hover:bg-accent"
      onClick={handleClick}
      aria-expanded={!group.isCollapsed}
    >
      <span className="text-[0.7rem] text-muted-foreground">
        {group.isCollapsed ? "▶" : "▼"}
      </span>
      <span className="font-semibold text-foreground">
        {group.displayValue}
      </span>
      <span className="text-xs text-muted-foreground bg-accent px-1.5 py-0.5 rounded-full">
        {group.count}
      </span>
    </button>
  );
}
