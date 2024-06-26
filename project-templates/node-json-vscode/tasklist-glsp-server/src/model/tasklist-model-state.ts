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
import { DefaultModelState, JsonModelState } from '@eclipse-glsp/server';
import { inject, injectable } from 'inversify';
import { TasklistModel } from '../database/interfaces/component-tasklist-tasklist.js';
import { TaskListModelIndex } from './tasklist-model-index.js';
import { TaskList } from './tasklist-model.js';

@injectable()
export class TaskListModelState extends DefaultModelState implements JsonModelState<TaskList> {
    @inject(TaskListModelIndex)
    override readonly index: TaskListModelIndex;

    protected _world: TasklistModel;
    protected _taskList: TaskList;

    public log(msg: string) {
        console.log('state ' + msg);
    }
    get sourceModel(): TaskList {
        return this._taskList;
    }

    get worldModel(): TasklistModel {
        return this._world;
    }

    updateSourceTasklistModel(taskList: TasklistModel): void {
        this._world = taskList;
        this.log(this._world.id());
    }
    updateSourceModel(taskList: TaskList): void {
        this._taskList = taskList;
        this.index.indexTaskList(taskList);
    }
}
