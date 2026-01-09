program constraintSimple;

uses
  System.StartUpCopy,
  FMX.Forms,
  constraintSimple.fmain in 'constraintSimple.fmain.pas' {FMain},
  ConstraintParticle in 'ConstraintParticle.pas',
  ConstraintParticle.Renderer.FMX2D in 'ConstraintParticle.Renderer.FMX2D.pas',
  ConstraintParticle.Simulation.BasicDistance in 'ConstraintParticle.Simulation.BasicDistance.pas',
  ConstraintParticle.Simulation.SeparateCollision in 'ConstraintParticle.Simulation.SeparateCollision.pas',
  ConstraintParticle.Simulation.DistanceChain in 'ConstraintParticle.Simulation.DistanceChain.pas';

{$R *.res}

begin
  {$IFDEF DEBUG}
  ReportMemoryLeaksOnShutdown := true;
  {$ENDIF}
  Application.Initialize;
  Application.CreateForm(TFMain, FMain);
  Application.Run;
end.
