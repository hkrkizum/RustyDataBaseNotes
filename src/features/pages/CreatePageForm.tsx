import { useState } from "react";

interface CreatePageFormProps {
  onSubmit: (title: string) => Promise<unknown>;
}

export function CreatePageForm({ onSubmit }: CreatePageFormProps) {
  const [title, setTitle] = useState("");
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!title.trim() || submitting) return;

    setSubmitting(true);
    try {
      const result = await onSubmit(title.trim());
      if (result) {
        setTitle("");
      }
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <form className="flex gap-2 mb-4" onSubmit={handleSubmit}>
      <input
        className="flex-1 px-3 py-2 border border-input rounded-md text-sm focus:outline-none focus:border-ring focus:ring-2 focus:ring-ring/15 disabled:opacity-60"
        type="text"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        placeholder="新しいページのタイトル..."
        maxLength={255}
        disabled={submitting}
      />
      <button
        className="px-4 py-2 bg-primary text-primary-foreground border-none rounded-md text-sm cursor-pointer hover:bg-primary/85 disabled:opacity-60 disabled:cursor-not-allowed"
        type="submit"
        disabled={!title.trim() || submitting}
      >
        {submitting ? "作成中..." : "作成"}
      </button>
    </form>
  );
}
