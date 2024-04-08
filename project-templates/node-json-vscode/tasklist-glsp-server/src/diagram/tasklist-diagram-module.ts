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
import {
    ActionHandlerConstructor,
    BindingTarget,
    ComputedBoundsActionHandler,
    DiagramConfiguration,
    DiagramModule,
    GModelFactory,
    GModelIndex,
    InstanceMultiBinding,
    LabelEditValidator,
    ModelState,
    OperationHandlerConstructor,
    SourceModelStorage
} from '@eclipse-glsp/server';
import { injectable } from 'inversify';
import { hello } from '../database/hello.js';
import { CreateTaskHandler } from '../handler/create-task-node-handler.js';
import { CreateTransitionHandler } from '../handler/create-transition-handler.js';
import { DeleteElementHandler } from '../handler/delete-element-handler.js';
import { TaskListApplyLabelEditHandler } from '../handler/tasklist-apply-label-edit-handler.js';
import { TaskListChangeBoundsHandler } from '../handler/tasklist-change-bounds-handler.js';
import { TaskListLabelEditValidator } from '../handler/tasklist-label-edit-validator.js';
import { TaskListGModelFactory } from '../model/tasklist-gmodel-factory.js';
import { TaskListModelIndex } from '../model/tasklist-model-index.js';
import { TaskListModelState } from '../model/tasklist-model-state.js';
import { TaskListStorage } from '../model/tasklist-storage.js';
import { TaskListDiagramConfiguration } from './tasklist-diagram-configuration.js';

@injectable()
export class TaskListDiagramModule extends DiagramModule {
    readonly diagramType = 'tasklist-diagram';
    private world = hello.Hello.createWorld();

    public log(msg: string) {
        console.log(msg);
    }

    protected bindDiagramConfiguration(): BindingTarget<DiagramConfiguration> {
        this.log(this.world.calls());
        return TaskListDiagramConfiguration;
    }

    protected bindSourceModelStorage(): BindingTarget<SourceModelStorage> {
        this.log(this.world.calls());
        return TaskListStorage;
    }

    protected bindModelState(): BindingTarget<ModelState> {
        this.log(this.world.calls());
        return { service: TaskListModelState };
    }

    protected bindGModelFactory(): BindingTarget<GModelFactory> {
        this.log(this.world.calls());
        return TaskListGModelFactory;
    }

    protected override configureActionHandlers(binding: InstanceMultiBinding<ActionHandlerConstructor>): void {
        this.log(this.world.calls());
        super.configureActionHandlers(binding);
        binding.add(ComputedBoundsActionHandler);
    }

    protected override configureOperationHandlers(binding: InstanceMultiBinding<OperationHandlerConstructor>): void {
        this.log(this.world.calls());
        super.configureOperationHandlers(binding);
        binding.add(CreateTaskHandler);
        binding.add(CreateTransitionHandler);
        binding.add(TaskListChangeBoundsHandler);
        binding.add(TaskListApplyLabelEditHandler);
        binding.add(DeleteElementHandler);
    }

    protected override bindGModelIndex(): BindingTarget<GModelIndex> {
        this.log(this.world.calls());
        this.context.bind(TaskListModelIndex).toSelf().inSingletonScope();
        return { service: TaskListModelIndex };
    }

    protected override bindLabelEditValidator(): BindingTarget<LabelEditValidator> | undefined {
        this.log(this.world.calls());
        return TaskListLabelEditValidator;
    }
}
