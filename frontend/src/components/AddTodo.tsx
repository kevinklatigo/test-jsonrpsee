import { useState } from "react";

interface AddTodoProps {
  onAdd: (text: string) => Promise<void>;
}

export function AddTodo({ onAdd }: AddTodoProps) {
  const [text, setText] = useState("");
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = text.trim();
    if (!trimmed) return;
    setSubmitting(true);
    try {
      await onAdd(trimmed);
      setText("");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="add-todo">
      <input
        type="text"
        value={text}
        onChange={(e) => setText(e.target.value)}
        placeholder="What needs to be done?"
        disabled={submitting}
      />
      <button type="submit" disabled={submitting || !text.trim()}>
        Add
      </button>
    </form>
  );
}
