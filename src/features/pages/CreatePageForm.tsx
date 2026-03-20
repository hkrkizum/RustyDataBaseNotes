import { useState } from "react";
import styles from "./CreatePageForm.module.css";

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
    <form className={styles.form} onSubmit={handleSubmit}>
      <input
        className={styles.input}
        type="text"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        placeholder="新しいページのタイトル..."
        maxLength={255}
        disabled={submitting}
      />
      <button
        className={styles.button}
        type="submit"
        disabled={!title.trim() || submitting}
      >
        {submitting ? "作成中..." : "作成"}
      </button>
    </form>
  );
}
