; ModuleID = 'probe5.bdb889f61c2d1eff-cgu.0'
source_filename = "probe5.bdb889f61c2d1eff-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

; std::f64::<impl f64>::copysign
; Function Attrs: inlinehint uwtable
define internal double @"_ZN3std3f6421_$LT$impl$u20$f64$GT$8copysign17h2040516d891ec92bE"(double %self, double %sign) unnamed_addr #0 {
start:
  %0 = alloca double, align 8
  %1 = call double @llvm.copysign.f64(double %self, double %sign)
  store double %1, ptr %0, align 8
  %_0 = load double, ptr %0, align 8, !noundef !2
  ret double %_0
}

; probe5::probe
; Function Attrs: uwtable
define void @_ZN6probe55probe17h4a57e4314036a9a1E() unnamed_addr #1 {
start:
; call std::f64::<impl f64>::copysign
  %_1 = call double @"_ZN3std3f6421_$LT$impl$u20$f64$GT$8copysign17h2040516d891ec92bE"(double 1.000000e+00, double -1.000000e+00)
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.copysign.f64(double, double) #2

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.75.0 (82e1608df 2023-12-21)"}
!2 = !{}
