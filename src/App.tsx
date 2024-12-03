import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type Todo =  string;

function App() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [todoInput, setTodoInput] = useState("");


  async function addTodo() {
    if (todoInput.trim() === "") return;
    const updatedTodos = await invoke<Todo[]>("add_todo", { task: todoInput });
    setTodos(updatedTodos);
    setTodoInput(""); // Clear input
  }

  async function deleteTodo(index: number) {
    const updatedTodos = await invoke<Todo[]>("delete_todo", { index });
    setTodos(updatedTodos);
  }

  useEffect(() => {
    async function fetchTodos() {
      const fetchedTodos = await invoke<Todo[]>("get_todos");
      setTodos(fetchedTodos);
    }

    
    fetchTodos();
  }, [])

  return (
    <main className="container">
      <h1>Todo App</h1>

      <div className="todo-input">
        <input
          type="text"
          value={todoInput}
          onChange={(e) => setTodoInput(e.target.value)}
          placeholder="Enter a task..."
        />
        <button onClick={addTodo}>Add Task</button>
      </div>

      <ul className="todo-list">
        {todos.map((todo, index) => (
          <li key={index}>
            {todo}
            <button onClick={() => deleteTodo(index)}>Delete</button>
          </li>
        ))}
      </ul>
    </main>
  );
}


export default App;
