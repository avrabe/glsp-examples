/********************************************************************************
 * Copyright (c) 2022-2023 EclipseSource and ot

export function createTask(relativeLocation: Point) {
    throw new Error('Function not implemented.');
}
hers.
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
import { Command, CreateNodeOperation, DefaultTypes, JsonCreateNodeOperationHandler, MaybePromise, Point } from '@eclipse-glsp/server';
import { inject, injectable } from 'inversify';
import { TaskListModelState } from '../model/tasklist-model-state';

import { tasklist } from './graph';
// import { ComponentGraphTasklist, Task } from './interfaces/component-graph-tasklist.d';

@injectable()
export class CreateTaskHandler extends JsonCreateNodeOperationHandler {
    readonly elementTypeIds = [DefaultTypes.NODE];

    @inject(TaskListModelState)
    protected override modelState: TaskListModelState;

    override createCommand(operation: CreateNodeOperation): MaybePromise<Command | undefined> {
        return this.commandOf(() => {
            const relativeLocation = this.getRelativeLocation(operation) ?? Point.ORIGIN;
            const task = tasklist.addTask(relativeLocation);
            // const task = this.createTask(relativeLocation);
            const taskList = this.modelState.sourceModel;
            taskList.tasks.push(task);
        });
    }

    // protected createTask(position: Point): Task {
    //    const nodeCounter = this.modelState.index.getAllByClass(GNode).length;
    //    const helloWorldNode = helloWorld();
    //    return {
    //        id: uuid.v4(),
    //        name: helloWorldNode + `${nodeCounter}`,
    //        position
    //    };
    // }

    get label(): string {
        return 'Task';
    }
}
