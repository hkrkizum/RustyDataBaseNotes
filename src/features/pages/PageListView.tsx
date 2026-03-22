import { PageItem } from "./PageItem";
import type { Page } from "./types";

interface PageListViewProps {
  pages: Page[];
  loading: boolean;
  onUpdateTitle: (id: string, title: string) => Promise<unknown>;
  onRequestDelete: (page: Page) => void;
  onPageClick: (page: Page) => void;
}

export function PageListView({
  pages,
  loading,
  onUpdateTitle,
  onRequestDelete,
  onPageClick,
}: PageListViewProps) {
  if (loading) {
    return (
      <div className="text-center p-8 text-muted-foreground">読み込み中...</div>
    );
  }

  if (pages.length === 0) {
    return (
      <div className="text-center p-8 text-muted-foreground">
        <p>ページがありません</p>
        <p className="text-sm mt-2">
          上のフォームから新しいページを作成してください
        </p>
      </div>
    );
  }

  return (
    <div className="border border-border rounded-md overflow-hidden">
      {pages.map((page) => (
        <PageItem
          key={page.id}
          page={page}
          onUpdateTitle={onUpdateTitle}
          onRequestDelete={onRequestDelete}
          onPageClick={onPageClick}
        />
      ))}
    </div>
  );
}
