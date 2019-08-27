//
//  MetalController.swift
//  vulkan_ios
//
//  Created by grenlight on 2018/12/17.
//

import UIKit

class MetalController: UIViewController {
    var displayLink: CADisplayLink?
    var drawObj: OpaquePointer?
    
    override func loadView() {
        self.view = MetalView()
    }
    
    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        // 在 viewDidLoad 里创建 wgpu 绘制对象会报 iOSurface 为 nil 的错误
        if drawObj == nil {
            if let metalView = self.view as? MetalView {
                drawObj = create_sdf_view(metalView.appView())
//                drawObj = create_page_turning(metalView.appView())
//                drawObj = create_fluid(metalView.appView())
//                drawObj = create_brush_view(metalView.appView())

//                enter_frame(drawObj)
                displayLink = CADisplayLink.init(target: self, selector: #selector(enterFrame))
                self.displayLink?.add(to: .current, forMode: .default)
            }
        } else {
            self.displayLink?.isPaused = false
        }
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        displayLink?.isPaused = true
    }
    
    @objc func enterFrame() {
        guard let obj = self.drawObj else {
            return
        }
        enter_frame(obj)
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let obj = self.drawObj {
            let p = touches.first!.location(in: self.view)
            let tp = TouchPoint(x: Float(p.x), y: Float(p.y), force: 0.0)
            touch_move(obj, tp)
        }
    }
    
    override func viewWillTransition(to size: CGSize, with coordinator: UIViewControllerTransitionCoordinator) {
        self.displayLink?.isPaused = true
        
        super.viewWillTransition(to: size, with: coordinator)
        
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.3) {
            [weak self] in
            if let obj = self?.drawObj {
                resize(obj)
                self?.displayLink?.isPaused = false
            }
        }
    }
    
    
    
    
    deinit {
        displayLink?.invalidate()
    }
    
}
