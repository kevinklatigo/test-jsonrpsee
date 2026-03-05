import { useEffect, useState } from "react";
import { useTodos } from "./hooks/useTodos";
import { AddTodo } from "./components/AddTodo";
import { TodoList } from "./components/TodoList";
import { onRpcLog, type RpcLog } from "./rpc-client";
import "./App.css";

function App() {
  const { todos, loading, error, addTodo, toggleTodo, removeTodo, clearCompleted } =
    useTodos();
  const [logs, setLogs] = useState<RpcLog[]>([]);

  useEffect(() => {
    onRpcLog((log) => {
      setLogs((prev) => [...prev.slice(-19), log]);
    });
  }, []);

  const completedCount = todos.filter((t) => t.done).length;

  return (
    <div className="app">
      <h1>JSON-RPC Todo App</h1>
      <p className="subtitle">
        React + Rust (jsonrpsee) over JSON-RPC 2.0
      </p>

      <AddTodo onAdd={addTodo} />

      {loading && <p>Loading...</p>}
      {error && <p className="error">{error}</p>}

      <TodoList todos={todos} onToggle={toggleTodo} onRemove={removeTodo} />

      {completedCount > 0 && (
        <button className="clear-btn" onClick={clearCompleted}>
          Clear {completedCount} completed
        </button>
      )}

      <details className="inspector" open>
        <summary>JSON-RPC Inspector ({logs.length} calls)</summary>
        <div className="inspector-logs">
          {logs.length === 0 && (
            <p className="empty">RPC calls will appear here...</p>
          )}
          {logs.map((log, i) => (
            <div key={i} className="log-entry">
              <div className="log-req">
                <strong>REQ</strong>
                <pre>{JSON.stringify(log.request, null, 2)}</pre>
              </div>
              <div className="log-res">
                <strong>RES</strong>
                <pre>{JSON.stringify(log.response, null, 2)}</pre>
              </div>
            </div>
          ))}
        </div>
      </details>
    </div>
  );
}

export default App;
