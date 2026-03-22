import { useCallback } from "react";
import styles from "./GroupHeader.module.css";
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
      className={styles.groupHeader}
      onClick={handleClick}
      aria-expanded={!group.isCollapsed}
    >
      <span className={styles.toggle}>{group.isCollapsed ? "▶" : "▼"}</span>
      <span className={styles.label}>{group.displayValue}</span>
      <span className={styles.count}>{group.count}</span>
    </button>
  );
}
