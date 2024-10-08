/********************************************************************************
 * Copyright (c) 2022 EclipseSource and others.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License v. 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0.
 *
 * This Source Code may also be made available under the following Secondary
 * Licenses when the conditions for such availability set forth in the Eclipse
 * Public License v. 2.0 are satisfied:
 * -- GNU General Public License, version 2 with the GNU Classpath Exception
 * which is available at https://www.gnu.org/software/classpath/license.html
 * -- MIT License which is available at https://opensource.org/license/mit.
 *
 * SPDX-License-Identifier: EPL-2.0 OR GPL-2.0 WITH Classpath-exception-2.0 OR MIT
 ********************************************************************************/
import { GEdge, GGraph, GLabel, GModelFactory, GNode } from '@eclipse-glsp/server';
import { inject, injectable } from 'inversify';
import { TaskListModelState } from './tasklist-model-state.js';
import { Task, Transition } from './tasklist-model.js';

@injectable()
export class TaskListGModelFactory implements GModelFactory {
    @inject(TaskListModelState)
    protected modelState: TaskListModelState;

    async createModel(): Promise<void> {
        const work_tasklist = this.modelState.sourceModel;
        const work_tasks: Array<Task> = work_tasklist.tasks();
        const childNodes = work_tasks.map(task => this.createTaskNode(task));
        const childEdges = work_tasklist.transitions().map(transition => this.createTransitionEdge(transition));

        const newRoot = GGraph.builder() //
            .id(work_tasklist.id())
            .addChildren(childNodes)
            .addChildren(childEdges)
            .build();
        this.modelState.updateRoot(newRoot);
    }
    protected createTaskNode(task: Task): GNode {
        const builder = GNode.builder()
            .id(task.id)
            .addCssClass('tasklist-node')
            .add(GLabel.builder().text(task.name).id(`${task.id}_label`).build())
            .layout('hbox')
            .addLayoutOption('paddingLeft', 5)
            .position(task.position);

        if (task.size) {
            builder.addLayoutOptions({ prefWidth: task.size.width, prefHeight: task.size.height });
        }

        return builder.build();
    }

    protected createTransitionEdge(transition: Transition): GEdge {
        return GEdge.builder() //
            .id(transition.id)
            .addCssClass('tasklist-transition')
            .sourceId(transition.sourceTaskId)
            .targetId(transition.targetTaskId)
            .build();
    }
}
