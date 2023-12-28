export namespace ComponentGraphTasklist {
  export function tasks(): Task[];
  export function addTask(position: Position): Task;
}
export interface Position {
  x: number,
  y: number,
}
export interface Size {
  width: number,
  height: number,
}
export interface Task {
  id: string,
  name: string,
  position: Position,
  size?: Size,
}
