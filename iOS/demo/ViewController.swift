//
//  ViewController.swift
//  brush
//
//  Created by grenlight on 2019/4/24.
//  Copyright © 2019 grenlight. All rights reserved.
//

import UIKit
import SnapKit

class ViewController: UIViewController {

    var touchPoints: Array<CGPoint> = []
    var lastTime: Date = Date()
    var timeIntervals: Array<Double> = []
    
    lazy var metalController: MetalController = {
        let controller = MetalController()
        self.addChild(controller)
        
        return controller
    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
        self.view.addSubview(metalController.view)
        metalController.view.snp.makeConstraints { (make) -> Void in
            make.edges.equalTo(self.view)
        }
                
    }
    
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        lastTime = Date()
        timeIntervals.append(0.0)
        touchPoints.append(touches.first!.location(in: self.view))
    }
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        let curTime = Date()
        timeIntervals.append(curTime.timeIntervalSince(lastTime))
        touchPoints.append(touches.first!.location(in: self.view))
        lastTime = curTime
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        let curTime = Date()
        timeIntervals.append(curTime.timeIntervalSince(lastTime))
        touchPoints.append(touches.first!.location(in: self.view))
        print("-- \(touchPoints.count) --")
        for p in touchPoints {
            print("[\(p.x), \(p.y)],")
        }
        print("\(timeIntervals)")
    }


}

