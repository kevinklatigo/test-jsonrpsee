const RPC_URL = import.meta.env.VITE_RPC_URL ?? "http://localhost:3000";

let nextId = 1;

export interface Todo {
  id: number;
  text: string;
  done: boolean;
}

export interface RpcSchema {
  todo_list: { params: void; result: Todo[] };
  todo_add: { params: { text: string }; result: Todo };
  todo_toggle: { params: { id: number }; result: Todo };
  todo_remove: { params: { id: number }; result: boolean };
  todo_clearCompleted: { params: void; result: number };
}

type RpcMethod = keyof RpcSchema;
type RpcParams<M extends RpcMethod> = RpcSchema[M]["params"];
type RpcResult<M extends RpcMethod> = RpcSchema[M]["result"];

type RpcCallArgs<M extends RpcMethod> =
  RpcParams<M> extends void ? [method: M] : [method: M, params: RpcParams<M>];

export async function rpcCall<M extends RpcMethod>(
  ...args: RpcCallArgs<M>
): Promise<RpcResult<M>> {
  const [method, params] = args;
  const id = nextId++;
  const request = {
    jsonrpc: "2.0" as const,
    method,
    params,
    id,
  };

  const res = await fetch(RPC_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request),
  });

  const response: JsonRpcResponse<RpcResult<M>> = await res.json();

  if (logCallback) {
    logCallback({ request, response, timestamp: Date.now() });
  }

  if ("error" in response) {
    throw new Error(
      `RPC Error ${response.error.code}: ${response.error.message}`
    );
  }

  return response.result;
}

interface JsonRpcSuccess<T> {
  jsonrpc: "2.0";
  result: T;
  id: number;
}

interface JsonRpcError {
  jsonrpc: "2.0";
  error: { code: number; message: string; data?: unknown };
  id: number | null;
}

type JsonRpcResponse<T> = JsonRpcSuccess<T> | JsonRpcError;

export interface RpcLog {
  request: object;
  response: object;
  timestamp: number;
}

type LogCallback = (log: RpcLog) => void;

let logCallback: LogCallback | null = null;

export function onRpcLog(cb: LogCallback) {
  logCallback = cb;
}
