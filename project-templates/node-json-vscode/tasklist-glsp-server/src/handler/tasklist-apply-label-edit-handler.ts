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
import { ApplyLabelEditOperation } from '@eclipse-glsp/protocol';
import { Command, GLSPServerError, GNode, JsonOperationHandler, MaybePromise, toTypeGuard } from '@eclipse-glsp/server/node.js';
import { inject, injectable } from 'inversify';
import { TaskListModelState } from '../model/tasklist-model-state.js';

@injectable()
export class TaskListApplyLabelEditHandler extends JsonOperationHandler {
    readonly operationType = ApplyLabelEditOperation.KIND;

    @inject(TaskListModelState)
    protected override readonly modelState: TaskListModelState;

    override createCommand(operation: ApplyLabelEditOperation): MaybePromise<Command | undefined> {
        return this.commandOf(() => {
            this.modelState.worldModel.setTaskName(operation.labelId, operation.text);
            const index = this.modelState.index;
            // Retrieve the parent node of the label that should be edited
            const taskNode = index.findParentElement(operation.labelId, toTypeGuard(GNode));
            if (taskNode) {
                const task = index.findTask(taskNode.id);
                if (!task) {
                    throw new GLSPServerError(`Could not retrieve the parent task for the label with id ${operation.labelId}`);
                }
                task.name = operation.text;
            }
        });
    }
}
