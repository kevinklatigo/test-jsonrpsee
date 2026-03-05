import { useCallback, useEffect, useState } from "react";
import { rpcCall, type Todo } from "../rpc-client";

export type { Todo };

export function useTodos() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchTodos = useCallback(async () => {
    try {
      const list = await rpcCall("todo_list");
      setTodos(list);
      setError(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to fetch todos");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchTodos();
  }, [fetchTodos]);

  const addTodo = useCallback(
    async (text: string) => {
      await rpcCall("todo_add", { text });
      await fetchTodos();
    },
    [fetchTodos]
  );

  const toggleTodo = useCallback(
    async (id: number) => {
      await rpcCall("todo_toggle", { id });
      await fetchTodos();
    },
    [fetchTodos]
  );

  const removeTodo = useCallback(
    async (id: number) => {
      await rpcCall("todo_remove", { id });
      await fetchTodos();
    },
    [fetchTodos]
  );

  const clearCompleted = useCallback(async () => {
    await rpcCall("todo_clearCompleted");
    await fetchTodos();
  }, [fetchTodos]);

  return { todos, loading, error, addTodo, toggleTodo, removeTodo, clearCompleted };
}
