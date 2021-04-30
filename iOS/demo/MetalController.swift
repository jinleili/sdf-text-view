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
    
    lazy var pintch: UIPinchGestureRecognizer = {
        let gestureRecognizer = UIPinchGestureRecognizer.init(target: self, action: #selector(pintchGesture))
        return gestureRecognizer
    }()
    
    override func loadView() {
        self.view = MetalView()
        self.view.addGestureRecognizer(pintch)
    }
    
    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        
        // 在 viewDidLoad 里创建 wgpu 绘制对象会报 iOSurface 为 nil 的错误
        if drawObj == nil {
            if let metalView = self.view as? MetalView {
                if let metalLayer = metalView.layer as? CAMetalLayer {
                    if let device = metalLayer.device {
                        if device.supportsFeatureSet(.iOS_GPUFamily2_v1) {
                            print("iOS_GPUFamily2_v1")
                        }
                        if device.supportsFeatureSet(.iOS_GPUFamily2_v2) {
                            print("iOS_GPUFamily2_v2")
                        }
                        if device.supportsFeatureSet(.iOS_GPUFamily2_v3) {
                            print("iOS_GPUFamily2_v3")
                        }
                        if device.supportsFeatureSet(.iOS_GPUFamily1_v3) {
                            print("iOS_GPUFamily1_v3")
                        }
                        if #available(iOS 13.0, *) {
                            if device.supportsFamily(.apple3) {
                                print("apple3")
                            }
                            if device.supportsFamily(.apple3) {
                                print("apple3")
                            }
                            if device.supportsFamily(.apple3) {
                                print("apple3")
                            }
                        } else {
                            // Fallback on earlier versions
                        }
                        
                    }
                    
                }
                
                let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(metalView).toOpaque())
                let metalLayer = UnsafeMutableRawPointer(Unmanaged.passRetained(metalView.layer).toOpaque())
                let maximumFrames = UIScreen.main.maximumFramesPerSecond
                let av = app_view(view: ownedPointer, metal_layer: metalLayer,maximum_frames: Int32(maximumFrames), temporary_directory: temporary_directory, callback_to_swift: callback_to_swift)
                drawObj = create_sdf_view(av)
                sdf_view_set_bundle_image(drawObj, UnsafeMutablePointer(mutating: "math0.png"))

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
    
    func generateTouchPoint(_ p: CGPoint) -> TouchPoint {
        return TouchPoint(x: Float(p.x), y: Float(p.y), azimuth_angle: 0.0, altitude_angle: 0.0, force: 0.0, stamp: 0.0, distance: 0.0, interval: 0.0, speed: 0.0, ty: 0, stamp_scale: 0.0)
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let obj = self.drawObj {
            let p = touches.first!.location(in: self.view)
            let tp = generateTouchPoint(p)
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
    
    @IBAction func pintchGesture(_ gestureRecognizer : UIPinchGestureRecognizer) {
        guard gestureRecognizer.view != nil else { return }
        
        if gestureRecognizer.state == .began || gestureRecognizer.state == .changed {
            let s = gestureRecognizer.scale
            if let obj = self.drawObj {
                self.displayLink?.isPaused = false
                let p = gestureRecognizer.location(in: self.view)
                let location = generateTouchPoint(p)
                pintch_changed(obj, location, Float(s))
                self.displayLink?.isPaused = false
            }
            gestureRecognizer.scale = 1.0
        }
        
    }

    
    
    deinit {
        displayLink?.invalidate()
    }
    
}
